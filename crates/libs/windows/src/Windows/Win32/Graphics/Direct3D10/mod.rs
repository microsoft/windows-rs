#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileEffectFromMemory<'a, P0, P1>(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: P0, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: P1, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrors: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D::ID3DInclude>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CompileEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: ::windows::core::PCSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut *mut ::core::ffi::c_void, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3D10CompileEffectFromMemory(::core::mem::transmute(pdata), datalength, psrcfilename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), hlslflags, fxflags, ::core::mem::transmute(ppcompiledeffect), ::core::mem::transmute(pperrors)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileShader<'a, P0, P1, P2, P3, P4>(psrcdata: P0, srcdatasize: usize, pfilename: P1, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: P2, pfunctionname: P3, pprofile: P4, flags: u32, ppshader: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D::ID3DInclude>>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
    P4: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CompileShader(psrcdata: ::windows::core::PCSTR, srcdatasize: usize, pfilename: ::windows::core::PCSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, pfunctionname: ::windows::core::PCSTR, pprofile: ::windows::core::PCSTR, flags: u32, ppshader: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3D10CompileShader(psrcdata.into(), srcdatasize, pfilename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), pfunctionname.into(), pprofile.into(), flags, ::core::mem::transmute(ppshader), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CreateBlob(numbytes: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateBlob(numbytes: usize, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateBlob(numbytes, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D10CreateDevice<'a, P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, sdkversion: u32) -> ::windows::core::Result<ID3D10Device>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dxgi::IDXGIAdapter>>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateDevice(padapter: *mut ::core::ffi::c_void, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateDevice(padapter.into().abi(), drivertype, software.into(), flags, sdkversion, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Device>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D10CreateDevice1<'a, P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32) -> ::windows::core::Result<ID3D10Device1>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dxgi::IDXGIAdapter>>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateDevice1(padapter: *mut ::core::ffi::c_void, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateDevice1(padapter.into().abi(), drivertype, software.into(), flags, hardwarelevel, sdkversion, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Device1>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain<'a, P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, ppdevice: *mut ::core::option::Option<ID3D10Device>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dxgi::IDXGIAdapter>>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateDeviceAndSwapChain(padapter: *mut ::core::ffi::c_void, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3D10CreateDeviceAndSwapChain(padapter.into().abi(), drivertype, software.into(), flags, sdkversion, ::core::mem::transmute(pswapchaindesc), ::core::mem::transmute(ppswapchain), ::core::mem::transmute(ppdevice)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain1<'a, P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, ppdevice: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dxgi::IDXGIAdapter>>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateDeviceAndSwapChain1(padapter: *mut ::core::ffi::c_void, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3D10CreateDeviceAndSwapChain1(padapter.into().abi(), drivertype, software.into(), flags, hardwarelevel, sdkversion, ::core::mem::transmute(pswapchaindesc), ::core::mem::transmute(ppswapchain), ::core::mem::transmute(ppdevice)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10CreateEffectFromMemory<'a, P0, P1>(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P0, peffectpool: P1) -> ::windows::core::Result<ID3D10Effect>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10EffectPool>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: *mut ::core::ffi::c_void, peffectpool: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateEffectFromMemory(::core::mem::transmute(pdata), datalength, fxflags, pdevice.into().abi(), peffectpool.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Effect>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10CreateEffectPoolFromMemory<'a, P0>(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P0) -> ::windows::core::Result<ID3D10EffectPool>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateEffectPoolFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: *mut ::core::ffi::c_void, ppeffectpool: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateEffectPoolFromMemory(::core::mem::transmute(pdata), datalength, fxflags, pdevice.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10EffectPool>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10CreateStateBlock<'a, P0>(pdevice: P0, pstateblockmask: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<ID3D10StateBlock>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10CreateStateBlock(pdevice: *mut ::core::ffi::c_void, pstateblockmask: *const D3D10_STATE_BLOCK_MASK, ppstateblock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10CreateStateBlock(pdevice.into().abi(), ::core::mem::transmute(pstateblockmask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10StateBlock>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10DisassembleEffect<'a, P0, P1>(peffect: P0, enablecolorcode: P1) -> ::windows::core::Result<super::Direct3D::ID3DBlob>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Effect>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10DisassembleEffect(peffect: *mut ::core::ffi::c_void, enablecolorcode: super::super::Foundation::BOOL, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10DisassembleEffect(peffect.into().abi(), enablecolorcode.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10DisassembleShader<'a, P0, P1>(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: P0, pcomments: P1) -> ::windows::core::Result<super::Direct3D::ID3DBlob>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10DisassembleShader(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: super::super::Foundation::BOOL, pcomments: ::windows::core::PCSTR, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10DisassembleShader(::core::mem::transmute(pshader), bytecodelength, enablecolorcode.into(), pcomments.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10GetGeometryShaderProfile<'a, P0>(pdevice: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetGeometryShaderProfile(pdevice: *mut ::core::ffi::c_void) -> ::windows::core::PSTR;
    }
    D3D10GetGeometryShaderProfile(pdevice.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10GetInputAndOutputSignatureBlob(::core::mem::transmute(pshaderbytecode), bytecodelength, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10GetInputSignatureBlob(::core::mem::transmute(pshaderbytecode), bytecodelength, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10GetOutputSignatureBlob(::core::mem::transmute(pshaderbytecode), bytecodelength, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10GetPixelShaderProfile<'a, P0>(pdevice: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetPixelShaderProfile(pdevice: *mut ::core::ffi::c_void) -> ::windows::core::PSTR;
    }
    D3D10GetPixelShaderProfile(pdevice.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppdebuginfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10GetShaderDebugInfo(::core::mem::transmute(pshaderbytecode), bytecodelength, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10GetVertexShaderProfile<'a, P0>(pdevice: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Device>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10GetVertexShaderProfile(pdevice: *mut ::core::ffi::c_void) -> ::windows::core::PSTR;
    }
    D3D10GetVertexShaderProfile(pdevice.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10PreprocessShader<'a, P0, P1, P2>(psrcdata: P0, srcdatasize: usize, pfilename: P1, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: P2, ppshadertext: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D::ID3DInclude>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10PreprocessShader(psrcdata: ::windows::core::PCSTR, srcdatasize: usize, pfilename: ::windows::core::PCSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, ppshadertext: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3D10PreprocessShader(psrcdata.into(), srcdatasize, pfilename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), ::core::mem::transmute(ppshadertext), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10ShaderReflection> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10ReflectShader(::core::mem::transmute(pshaderbytecode), bytecodelength, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderReflection>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10StateBlockMaskDifference(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableAll() -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10StateBlockMaskDisableAll(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::HRESULT;
    }
    D3D10StateBlockMaskDisableCapture(::core::mem::transmute(pmask), statetype, rangestart, rangelength).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableAll() -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10StateBlockMaskEnableAll(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::HRESULT;
    }
    D3D10StateBlockMaskEnableCapture(::core::mem::transmute(pmask), statetype, rangestart, rangelength).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL;
    }
    D3D10StateBlockMaskGetSetting(::core::mem::transmute(pmask), statetype, entry)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10StateBlockMaskIntersect(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[inline]
pub unsafe fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3D10StateBlockMaskUnion(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_APPNAME_STRING: &str = "Name";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_APPSIZE_STRING: &str = "Size";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_ASYNC_GETDATA_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ASYNC_GETDATA_DONOTFLUSH: D3D10_ASYNC_GETDATA_FLAG = D3D10_ASYNC_GETDATA_FLAG(1i32);
impl ::core::marker::Copy for D3D10_ASYNC_GETDATA_FLAG {}
impl ::core::clone::Clone for D3D10_ASYNC_GETDATA_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_ASYNC_GETDATA_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_ASYNC_GETDATA_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_ASYNC_GETDATA_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_ASYNC_GETDATA_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_BIND_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_VERTEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_INDEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_CONSTANT_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_SHADER_RESOURCE: D3D10_BIND_FLAG = D3D10_BIND_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_STREAM_OUTPUT: D3D10_BIND_FLAG = D3D10_BIND_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_RENDER_TARGET: D3D10_BIND_FLAG = D3D10_BIND_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BIND_DEPTH_STENCIL: D3D10_BIND_FLAG = D3D10_BIND_FLAG(64i32);
impl ::core::marker::Copy for D3D10_BIND_FLAG {}
impl ::core::clone::Clone for D3D10_BIND_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_BIND_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_BIND_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_BIND_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BIND_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_BLEND(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_ZERO: D3D10_BLEND = D3D10_BLEND(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_ONE: D3D10_BLEND = D3D10_BLEND(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_SRC_COLOR: D3D10_BLEND = D3D10_BLEND(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_SRC_COLOR: D3D10_BLEND = D3D10_BLEND(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_SRC_ALPHA: D3D10_BLEND = D3D10_BLEND(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_SRC_ALPHA: D3D10_BLEND = D3D10_BLEND(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_DEST_ALPHA: D3D10_BLEND = D3D10_BLEND(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_DEST_ALPHA: D3D10_BLEND = D3D10_BLEND(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_DEST_COLOR: D3D10_BLEND = D3D10_BLEND(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_DEST_COLOR: D3D10_BLEND = D3D10_BLEND(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_SRC_ALPHA_SAT: D3D10_BLEND = D3D10_BLEND(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_BLEND_FACTOR: D3D10_BLEND = D3D10_BLEND(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_BLEND_FACTOR: D3D10_BLEND = D3D10_BLEND(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_SRC1_COLOR: D3D10_BLEND = D3D10_BLEND(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_SRC1_COLOR: D3D10_BLEND = D3D10_BLEND(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_SRC1_ALPHA: D3D10_BLEND = D3D10_BLEND(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_INV_SRC1_ALPHA: D3D10_BLEND = D3D10_BLEND(19i32);
impl ::core::marker::Copy for D3D10_BLEND {}
impl ::core::clone::Clone for D3D10_BLEND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_BLEND {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BLEND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub BlendEnable: [super::super::Foundation::BOOL; 8],
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: [u8; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BLEND_DESC").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("BlendEnable", &self.BlendEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_BLEND_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BLEND_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BLEND_DESC1").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("IndependentBlendEnable", &self.IndependentBlendEnable).field("RenderTarget", &self.RenderTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_BLEND_DESC1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BLEND_DESC1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_BLEND_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_OP_ADD: D3D10_BLEND_OP = D3D10_BLEND_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_OP_SUBTRACT: D3D10_BLEND_OP = D3D10_BLEND_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_OP_REV_SUBTRACT: D3D10_BLEND_OP = D3D10_BLEND_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_OP_MIN: D3D10_BLEND_OP = D3D10_BLEND_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BLEND_OP_MAX: D3D10_BLEND_OP = D3D10_BLEND_OP(5i32);
impl ::core::marker::Copy for D3D10_BLEND_OP {}
impl ::core::clone::Clone for D3D10_BLEND_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_BLEND_OP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_BLEND_OP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_BLEND_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BLEND_OP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl ::core::marker::Copy for D3D10_BOX {}
impl ::core::clone::Clone for D3D10_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BOX").field("left", &self.left).field("top", &self.top).field("front", &self.front).field("right", &self.right).field("bottom", &self.bottom).field("back", &self.back).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_BOX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BOX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BOX>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BOX {}
impl ::core::default::Default for D3D10_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BREAKON_CATEGORY: &str = "BreakOn_CATEGORY_%s";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BREAKON_ID_DECIMAL: &str = "BreakOn_ID_%d";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BREAKON_ID_STRING: &str = "BreakOn_ID_%s";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_BREAKON_SEVERITY: &str = "BreakOn_SEVERITY_%s";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_DESC {}
impl ::core::clone::Clone for D3D10_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BUFFER_DESC").field("ByteWidth", &self.ByteWidth).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_DESC {}
impl ::core::default::Default for D3D10_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_BUFFER_RTV {
    pub Anonymous1: D3D10_BUFFER_RTV_0,
    pub Anonymous2: D3D10_BUFFER_RTV_1,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_RTV {}
impl ::core::default::Default for D3D10_BUFFER_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub union D3D10_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV_0 {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_RTV_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_RTV_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_RTV_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_RTV_0 {}
impl ::core::default::Default for D3D10_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub union D3D10_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV_1 {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_RTV_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_RTV_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_RTV_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_RTV_1 {}
impl ::core::default::Default for D3D10_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_BUFFER_SRV {
    pub Anonymous1: D3D10_BUFFER_SRV_0,
    pub Anonymous2: D3D10_BUFFER_SRV_1,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_SRV {}
impl ::core::default::Default for D3D10_BUFFER_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub union D3D10_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV_0 {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_SRV_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_SRV_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_SRV_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_SRV_0 {}
impl ::core::default::Default for D3D10_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub union D3D10_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV_1 {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for D3D10_BUFFER_SRV_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_SRV_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_BUFFER_SRV_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_SRV_1 {}
impl ::core::default::Default for D3D10_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_CLEAR_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CLEAR_DEPTH: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CLEAR_STENCIL: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(2i32);
impl ::core::marker::Copy for D3D10_CLEAR_FLAG {}
impl ::core::clone::Clone for D3D10_CLEAR_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_CLEAR_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_CLEAR_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_CLEAR_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CLEAR_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_COLOR_WRITE_ENABLE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COLOR_WRITE_ENABLE_RED: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COLOR_WRITE_ENABLE_GREEN: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COLOR_WRITE_ENABLE_BLUE: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COLOR_WRITE_ENABLE_ALPHA: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COLOR_WRITE_ENABLE_ALL: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(15i32);
impl ::core::marker::Copy for D3D10_COLOR_WRITE_ENABLE {}
impl ::core::clone::Clone for D3D10_COLOR_WRITE_ENABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_COLOR_WRITE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_COLOR_WRITE_ENABLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_FILTERING_BIT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_COMPARISON_FUNC(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_NEVER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_LESS: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_LESS_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_GREATER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_NOT_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_GREATER_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COMPARISON_ALWAYS: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(8i32);
impl ::core::marker::Copy for D3D10_COMPARISON_FUNC {}
impl ::core::clone::Clone for D3D10_COMPARISON_FUNC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_COMPARISON_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_COMPARISON_FUNC {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_COMPARISON_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COMPARISON_FUNC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_COUNTER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_GPU_IDLE: D3D10_COUNTER = D3D10_COUNTER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_VERTEX_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_PIXEL_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: D3D10_COUNTER = D3D10_COUNTER(1073741824i32);
impl ::core::marker::Copy for D3D10_COUNTER {}
impl ::core::clone::Clone for D3D10_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_COUNTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_COUNTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_COUNTER_DESC {
    pub Counter: D3D10_COUNTER,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_COUNTER_DESC {}
impl ::core::clone::Clone for D3D10_COUNTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_COUNTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_COUNTER_DESC").field("Counter", &self.Counter).field("MiscFlags", &self.MiscFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_COUNTER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_COUNTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_COUNTER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_COUNTER_DESC {}
impl ::core::default::Default for D3D10_COUNTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D10_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
impl ::core::marker::Copy for D3D10_COUNTER_INFO {}
impl ::core::clone::Clone for D3D10_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_COUNTER_INFO").field("LastDeviceDependentCounter", &self.LastDeviceDependentCounter).field("NumSimultaneousCounters", &self.NumSimultaneousCounters).field("NumDetectableParallelUnits", &self.NumDetectableParallelUnits).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_COUNTER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_COUNTER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_COUNTER_INFO {}
impl ::core::default::Default for D3D10_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_COUNTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TYPE_FLOAT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TYPE_UINT16: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TYPE_UINT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_COUNTER_TYPE_UINT64: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(3i32);
impl ::core::marker::Copy for D3D10_COUNTER_TYPE {}
impl ::core::clone::Clone for D3D10_COUNTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_COUNTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_CPU_ACCESS_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CPU_ACCESS_WRITE: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CPU_ACCESS_READ: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(131072i32);
impl ::core::marker::Copy for D3D10_CPU_ACCESS_FLAG {}
impl ::core::clone::Clone for D3D10_CPU_ACCESS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_CPU_ACCESS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_CPU_ACCESS_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_CPU_ACCESS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CPU_ACCESS_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_CREATE_DEVICE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_DEBUG: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1024i32);
impl ::core::marker::Copy for D3D10_CREATE_DEVICE_FLAG {}
impl ::core::clone::Clone for D3D10_CREATE_DEVICE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_CREATE_DEVICE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_CREATE_DEVICE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_CREATE_DEVICE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CREATE_DEVICE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_CULL_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CULL_NONE: D3D10_CULL_MODE = D3D10_CULL_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CULL_FRONT: D3D10_CULL_MODE = D3D10_CULL_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CULL_BACK: D3D10_CULL_MODE = D3D10_CULL_MODE(3i32);
impl ::core::marker::Copy for D3D10_CULL_MODE {}
impl ::core::clone::Clone for D3D10_CULL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_CULL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_CULL_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_CULL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CULL_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_MAX_ANISOTROPY: f32 = 16f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
impl ::core::marker::Copy for D3D10_DEPTH_STENCILOP_DESC {}
impl ::core::clone::Clone for D3D10_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_DEPTH_STENCILOP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_DEPTH_STENCILOP_DESC").field("StencilFailOp", &self.StencilFailOp).field("StencilDepthFailOp", &self.StencilDepthFailOp).field("StencilPassOp", &self.StencilPassOp).field("StencilFunc", &self.StencilFunc).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_DEPTH_STENCILOP_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCILOP_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_DEPTH_STENCILOP_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_DEPTH_STENCILOP_DESC {}
impl ::core::default::Default for D3D10_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D10_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D10_DEPTH_STENCILOP_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_DEPTH_STENCIL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_DEPTH_STENCIL_DESC").field("DepthEnable", &self.DepthEnable).field("DepthWriteMask", &self.DepthWriteMask).field("DepthFunc", &self.DepthFunc).field("StencilEnable", &self.StencilEnable).field("StencilReadMask", &self.StencilReadMask).field("StencilWriteMask", &self.StencilWriteMask).field("FrontFace", &self.FrontFace).field("BackFace", &self.BackFace).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_DEPTH_STENCIL_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCIL_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_DEPTH_STENCIL_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_DSV_DIMENSION,
    pub Anonymous: D3D10_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_DEPTH_STENCIL_VIEW_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_DEPTH_STENCIL_VIEW_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D10_TEX1D_DSV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D10_TEX2D_DSV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D10_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_DEPTH_STENCIL_VIEW_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_DEPTH_WRITE_MASK(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(1i32);
impl ::core::marker::Copy for D3D10_DEPTH_WRITE_MASK {}
impl ::core::clone::Clone for D3D10_DEPTH_WRITE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_DEPTH_WRITE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_DEPTH_WRITE_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_DEVICE_STATE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_SO_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_OM_RENDER_TARGETS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_OM_BLEND_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_VS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_VS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_VS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_VS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_GS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_GS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_GS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_GS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_PS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_PS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_PS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_PS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_IA_VERTEX_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_IA_INDEX_BUFFER: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_IA_INPUT_LAYOUT: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_RS_VIEWPORTS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_RS_SCISSOR_RECTS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_RS_RASTERIZER_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DST_PREDICATION: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(24i32);
impl ::core::marker::Copy for D3D10_DEVICE_STATE_TYPES {}
impl ::core::clone::Clone for D3D10_DEVICE_STATE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_DEVICE_STATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_DEVICE_STATE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_DEVICE_STATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DEVICE_STATE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_DRIVER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(5i32);
impl ::core::marker::Copy for D3D10_DRIVER_TYPE {}
impl ::core::clone::Clone for D3D10_DRIVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_DRIVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_DRIVER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_DRIVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DRIVER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_DSV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(6i32);
impl ::core::marker::Copy for D3D10_DSV_DIMENSION {}
impl ::core::clone::Clone for D3D10_DSV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_DSV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_DSV_DIMENSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_DSV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DSV_DIMENSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: super::super::Foundation::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_EFFECT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_DESC").field("IsChildEffect", &self.IsChildEffect).field("ConstantBuffers", &self.ConstantBuffers).field("SharedConstantBuffers", &self.SharedConstantBuffers).field("GlobalVariables", &self.GlobalVariables).field("SharedGlobalVariables", &self.SharedGlobalVariables).field("Techniques", &self.Techniques).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_EFFECT_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_EFFECT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_EFFECT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: super::super::Foundation::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: ::windows::core::PCSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_SHADER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_EFFECT_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_SHADER_DESC").field("pInputSignature", &self.pInputSignature).field("IsInline", &self.IsInline).field("pBytecode", &self.pBytecode).field("BytecodeLength", &self.BytecodeLength).field("SODecl", &self.SODecl).field("NumInputSignatureEntries", &self.NumInputSignatureEntries).field("NumOutputSignatureEntries", &self.NumOutputSignatureEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_EFFECT_SHADER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_SHADER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_EFFECT_SHADER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: ::windows::core::PCSTR,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Elements: u32,
    pub Members: u32,
    pub Rows: u32,
    pub Columns: u32,
    pub PackedSize: u32,
    pub UnpackedSize: u32,
    pub Stride: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_EFFECT_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_EFFECT_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_EFFECT_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_TYPE_DESC").field("TypeName", &self.TypeName).field("Class", &self.Class).field("Type", &self.Type).field("Elements", &self.Elements).field("Members", &self.Members).field("Rows", &self.Rows).field("Columns", &self.Columns).field("PackedSize", &self.PackedSize).field("UnpackedSize", &self.UnpackedSize).field("Stride", &self.Stride).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_EFFECT_TYPE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_TYPE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_EFFECT_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Semantic: ::windows::core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
impl ::core::marker::Copy for D3D10_EFFECT_VARIABLE_DESC {}
impl ::core::clone::Clone for D3D10_EFFECT_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_EFFECT_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_VARIABLE_DESC").field("Name", &self.Name).field("Semantic", &self.Semantic).field("Flags", &self.Flags).field("Annotations", &self.Annotations).field("BufferOffset", &self.BufferOffset).field("ExplicitBindPoint", &self.ExplicitBindPoint).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_EFFECT_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_EFFECT_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_EFFECT_VARIABLE_DESC {}
impl ::core::default::Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ENABLE_BREAK_ON_MESSAGE: &str = "EnableBreakOnMessage";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_FEATURE_LEVEL1(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(40960i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(41216i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37120i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37376i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37632i32);
impl ::core::marker::Copy for D3D10_FEATURE_LEVEL1 {}
impl ::core::clone::Clone for D3D10_FEATURE_LEVEL1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_FEATURE_LEVEL1 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_FEATURE_LEVEL1 {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_FEATURE_LEVEL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FEATURE_LEVEL1").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_FILL_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILL_WIREFRAME: D3D10_FILL_MODE = D3D10_FILL_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILL_SOLID: D3D10_FILL_MODE = D3D10_FILL_MODE(3i32);
impl ::core::marker::Copy for D3D10_FILL_MODE {}
impl ::core::clone::Clone for D3D10_FILL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_FILL_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILL_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_MIN_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_ANISOTROPIC: D3D10_FILTER = D3D10_FILTER(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(149i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_COMPARISON_ANISOTROPIC: D3D10_FILTER = D3D10_FILTER(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_TEXT_1BIT: D3D10_FILTER = D3D10_FILTER(-2147483648i32);
impl ::core::marker::Copy for D3D10_FILTER {}
impl ::core::clone::Clone for D3D10_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_FILTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_TYPE_POINT: D3D10_FILTER_TYPE = D3D10_FILTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_TYPE_LINEAR: D3D10_FILTER_TYPE = D3D10_FILTER_TYPE(1i32);
impl ::core::marker::Copy for D3D10_FILTER_TYPE {}
impl ::core::clone::Clone for D3D10_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_FILTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FILTER_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_FORMAT_SUPPORT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SO_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_TEXTURE1D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_TEXTURE2D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_TEXTURE3D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_TEXTURECUBE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SHADER_LOAD: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2048i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_MIP: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4096i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_MIP_AUTOGEN: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_RENDER_TARGET: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16384i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_BLENDABLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(32768i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_DEPTH_STENCIL: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_CPU_LOCKABLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(131072i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(262144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_DISPLAY: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(524288i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1048576i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2097152i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4194304i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_SHADER_GATHER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8388608i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16777216i32);
impl ::core::marker::Copy for D3D10_FORMAT_SUPPORT {}
impl ::core::clone::Clone for D3D10_FORMAT_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_FORMAT_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_FORMAT_SUPPORT {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_INPUT_REGISTER_VERTICES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_OUTPUT_ELEMENTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INFOQUEUE_STORAGE_FILTER_OVERRIDE: &str = "InfoQueueStorageFilterOverride";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_INFO_QUEUE_FILTER {
    pub AllowList: D3D10_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D10_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for D3D10_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for D3D10_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_INFO_QUEUE_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_INFO_QUEUE_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_INFO_QUEUE_FILTER {}
impl ::core::default::Default for D3D10_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D10_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D10_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D10_MESSAGE_ID,
}
impl ::core::marker::Copy for D3D10_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for D3D10_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_INFO_QUEUE_FILTER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_INFO_QUEUE_FILTER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_INFO_QUEUE_FILTER_DESC {}
impl ::core::default::Default for D3D10_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_INPUT_CLASSIFICATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(1i32);
impl ::core::marker::Copy for D3D10_INPUT_CLASSIFICATION {}
impl ::core::clone::Clone for D3D10_INPUT_CLASSIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_INPUT_CLASSIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_INPUT_CLASSIFICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_INPUT_ELEMENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INPUT_ELEMENT_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Format", &self.Format).field("InputSlot", &self.InputSlot).field("AlignedByteOffset", &self.AlignedByteOffset).field("InputSlotClass", &self.InputSlotClass).field("InstanceDataStepRate", &self.InstanceDataStepRate).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_INPUT_ELEMENT_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_INPUT_ELEMENT_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_MAP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_READ: D3D10_MAP = D3D10_MAP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_WRITE: D3D10_MAP = D3D10_MAP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_READ_WRITE: D3D10_MAP = D3D10_MAP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_WRITE_DISCARD: D3D10_MAP = D3D10_MAP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_WRITE_NO_OVERWRITE: D3D10_MAP = D3D10_MAP(5i32);
impl ::core::marker::Copy for D3D10_MAP {}
impl ::core::clone::Clone for D3D10_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_MAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_MAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MAP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_MAPPED_TEXTURE2D {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: u32,
}
impl ::core::marker::Copy for D3D10_MAPPED_TEXTURE2D {}
impl ::core::clone::Clone for D3D10_MAPPED_TEXTURE2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_MAPPED_TEXTURE2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MAPPED_TEXTURE2D").field("pData", &self.pData).field("RowPitch", &self.RowPitch).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_MAPPED_TEXTURE2D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_MAPPED_TEXTURE2D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_MAPPED_TEXTURE2D>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_MAPPED_TEXTURE2D {}
impl ::core::default::Default for D3D10_MAPPED_TEXTURE2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_MAPPED_TEXTURE3D {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl ::core::marker::Copy for D3D10_MAPPED_TEXTURE3D {}
impl ::core::clone::Clone for D3D10_MAPPED_TEXTURE3D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_MAPPED_TEXTURE3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MAPPED_TEXTURE3D").field("pData", &self.pData).field("RowPitch", &self.RowPitch).field("DepthPitch", &self.DepthPitch).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_MAPPED_TEXTURE3D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_MAPPED_TEXTURE3D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_MAPPED_TEXTURE3D>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_MAPPED_TEXTURE3D {}
impl ::core::default::Default for D3D10_MAPPED_TEXTURE3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_MAP_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAP_FLAG_DO_NOT_WAIT: D3D10_MAP_FLAG = D3D10_MAP_FLAG(1048576i32);
impl ::core::marker::Copy for D3D10_MAP_FLAG {}
impl ::core::clone::Clone for D3D10_MAP_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_MAP_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_MAP_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_MAP_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MAP_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_DEPTH: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D10_MESSAGE {}
impl ::core::clone::Clone for D3D10_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MESSAGE").field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_MESSAGE {}
impl ::core::default::Default for D3D10_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_MESSAGE_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_MISCELLANEOUS: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_INITIALIZATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_CLEANUP: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_COMPILATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_STATE_CREATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_STATE_SETTING: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_STATE_GETTING: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_EXECUTION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_CATEGORY_SHADER: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(10i32);
impl ::core::marker::Copy for D3D10_MESSAGE_CATEGORY {}
impl ::core::clone::Clone for D3D10_MESSAGE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_MESSAGE_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_MESSAGE_ID(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_UNKNOWN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_STRING_FROM_APPLICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_THIS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(24i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(25i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(26i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(27i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(28i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(29i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(30i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(31i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(38i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(39i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(40i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(41i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(42i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(43i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(44i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(45i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(46i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(47i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(48i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(49i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(50i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(51i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(52i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(53i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(54i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(55i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(56i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(57i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(58i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(59i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(60i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(61i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(62i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(65i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(66i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(67i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(68i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(69i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(70i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(71i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(72i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(73i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(74i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(75i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(76i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(77i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(78i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(79i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(80i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(81i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(82i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(83i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(84i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(85i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(86i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(87i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(88i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(89i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(91i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(92i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(93i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(94i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(95i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(96i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(97i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(98i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(99i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(100i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(101i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(102i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(103i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(104i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(105i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(106i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(107i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(108i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(109i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(110i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(111i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(112i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(113i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(114i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(115i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(116i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(117i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(118i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(119i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(120i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(121i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(122i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(123i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(124i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(125i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(126i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(127i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(129i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(130i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(131i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(132i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(133i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(134i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(135i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(136i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(137i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(138i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(139i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(140i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(141i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(142i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(143i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(144i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(145i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(146i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(147i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(148i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(149i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(150i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(151i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(152i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(153i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(154i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(155i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(156i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(157i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(158i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(159i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(160i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(161i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(162i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(163i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(164i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(165i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(166i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(167i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(168i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(169i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(170i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(171i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(172i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(173i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(174i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(175i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(176i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(177i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(178i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(179i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(180i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(181i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(182i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(183i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(184i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(185i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(186i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(187i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(188i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(189i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(190i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(191i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(192i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(193i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(194i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(195i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(196i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(197i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(198i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(199i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(200i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(201i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(202i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(203i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(204i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(205i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(206i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(207i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(208i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(209i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(210i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(211i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(212i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(213i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(214i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(215i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(216i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(217i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(218i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(219i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(220i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(221i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(222i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(223i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(224i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(225i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(226i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(227i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(228i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(229i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(230i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(231i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(232i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(233i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(234i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(235i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(236i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(237i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(238i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(239i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(240i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(241i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(242i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(243i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(244i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(245i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(246i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(247i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(248i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(249i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(250i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(251i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(252i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(253i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(254i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(255i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(257i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(258i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(259i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(260i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(261i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(262i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(263i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(264i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(265i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(266i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(267i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(268i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(269i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(270i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(271i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(272i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(273i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(274i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(275i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(276i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(277i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(278i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(279i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(280i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(281i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(282i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(283i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(284i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(285i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(286i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(287i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(288i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(289i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(290i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(291i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(292i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(293i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(294i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(295i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(296i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(297i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(298i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(299i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(300i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(301i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(302i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(303i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(304i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(305i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(306i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(307i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(308i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(309i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(310i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(311i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(312i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(313i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(314i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(315i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(316i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(317i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(318i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(319i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(320i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(321i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(322i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(323i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(324i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(325i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(326i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_THREADING_MODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(327i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(328i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(329i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(330i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(331i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(332i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(333i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_REF_INFO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(334i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(335i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(336i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(337i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(338i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(339i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(340i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(341i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(342i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(343i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(344i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(345i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(346i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(347i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(348i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(349i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(350i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(351i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(352i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(353i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(354i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(355i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(356i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(357i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(358i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(359i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(360i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(361i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(362i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(363i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(364i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(365i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(366i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(367i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(368i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(369i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(370i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(371i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(372i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(373i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(374i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(375i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(376i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(377i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(378i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(379i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(380i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(381i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(382i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(383i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(384i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(385i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(386i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(387i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(388i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(389i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(390i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(391i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(392i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(393i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(394i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(395i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(396i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(397i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(398i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(399i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(400i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(401i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(402i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(403i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(404i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(405i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(406i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(407i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(408i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(409i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(410i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(411i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(412i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(413i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(414i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(415i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(416i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(417i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(418i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(419i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(420i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(421i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(422i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(423i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE1D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(424i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE2D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(425i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE3D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(426i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(427i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(428i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(429i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_VERTEXSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(430i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(431i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_PIXELSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(432i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(433i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_SAMPLER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(434i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_BLENDSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(435i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(436i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(437i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_QUERY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(438i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_PREDICATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(439i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(440i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_DEVICE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(441i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_LIVE_SWAPCHAIN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(442i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_D3D10_MESSAGES_END: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(443i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048576i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048577i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048578i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048579i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048580i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048581i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048582i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048583i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048584i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048585i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048586i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048587i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048588i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048589i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048590i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048591i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048592i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048593i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048594i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048595i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048596i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048597i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048598i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048599i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048600i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048601i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048602i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048603i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048604i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048605i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048606i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048607i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048608i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048609i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048610i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048611i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048612i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048613i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048614i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048615i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048616i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048617i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048618i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048619i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048620i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048621i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048622i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048623i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048624i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048625i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048626i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048627i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048628i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048629i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048630i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048631i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048632i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048633i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048634i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048635i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048636i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048637i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048638i32);
impl ::core::marker::Copy for D3D10_MESSAGE_ID {}
impl ::core::clone::Clone for D3D10_MESSAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_MESSAGE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_MESSAGE_SEVERITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_SEVERITY_CORRUPTION: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_SEVERITY_ERROR: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_SEVERITY_WARNING: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_SEVERITY_INFO: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MESSAGE_SEVERITY_MESSAGE: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(4i32);
impl ::core::marker::Copy for D3D10_MESSAGE_SEVERITY {}
impl ::core::clone::Clone for D3D10_MESSAGE_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_MESSAGE_SEVERITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIP_LOD_BIAS_MIN: f32 = -16f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MUTE_CATEGORY: &str = "Mute_CATEGORY_%s";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MUTE_DEBUG_OUTPUT: &str = "MuteDebugOutput";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MUTE_ID_DECIMAL: &str = "Mute_ID_%d";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MUTE_ID_STRING: &str = "Mute_ID_%s";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_MUTE_SEVERITY: &str = "Mute_SEVERITY_%s";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_PASS_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
impl ::core::marker::Copy for D3D10_PASS_DESC {}
impl ::core::clone::Clone for D3D10_PASS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_PASS_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_PASS_DESC").field("Name", &self.Name).field("Annotations", &self.Annotations).field("pIAInputSignature", &self.pIAInputSignature).field("IAInputSignatureSize", &self.IAInputSignatureSize).field("StencilRef", &self.StencilRef).field("SampleMask", &self.SampleMask).field("BlendFactor", &self.BlendFactor).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_PASS_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_PASS_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_PASS_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_PASS_DESC {}
impl ::core::default::Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: ::core::option::Option<ID3D10EffectShaderVariable>,
    pub ShaderIndex: u32,
}
impl ::core::clone::Clone for D3D10_PASS_SHADER_DESC {
    fn clone(&self) -> Self {
        Self { pShaderVariable: self.pShaderVariable.clone(), ShaderIndex: self.ShaderIndex }
    }
}
impl ::core::fmt::Debug for D3D10_PASS_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_PASS_SHADER_DESC").field("pShaderVariable", &self.pShaderVariable).field("ShaderIndex", &self.ShaderIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_PASS_SHADER_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for D3D10_PASS_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pShaderVariable == other.pShaderVariable && self.ShaderIndex == other.ShaderIndex
    }
}
impl ::core::cmp::Eq for D3D10_PASS_SHADER_DESC {}
impl ::core::default::Default for D3D10_PASS_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_QUERY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_EVENT: D3D10_QUERY = D3D10_QUERY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_OCCLUSION: D3D10_QUERY = D3D10_QUERY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_TIMESTAMP: D3D10_QUERY = D3D10_QUERY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: D3D10_QUERY = D3D10_QUERY(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_PIPELINE_STATISTICS: D3D10_QUERY = D3D10_QUERY(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_OCCLUSION_PREDICATE: D3D10_QUERY = D3D10_QUERY(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_SO_STATISTICS: D3D10_QUERY = D3D10_QUERY(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: D3D10_QUERY = D3D10_QUERY(7i32);
impl ::core::marker::Copy for D3D10_QUERY {}
impl ::core::clone::Clone for D3D10_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_QUERY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_QUERY {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_QUERY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
}
impl ::core::marker::Copy for D3D10_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::clone::Clone for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_PIPELINE_STATISTICS").field("IAVertices", &self.IAVertices).field("IAPrimitives", &self.IAPrimitives).field("VSInvocations", &self.VSInvocations).field("GSInvocations", &self.GSInvocations).field("GSPrimitives", &self.GSPrimitives).field("CInvocations", &self.CInvocations).field("CPrimitives", &self.CPrimitives).field("PSInvocations", &self.PSInvocations).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_QUERY_DATA_PIPELINE_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::default::Default for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl ::core::marker::Copy for D3D10_QUERY_DATA_SO_STATISTICS {}
impl ::core::clone::Clone for D3D10_QUERY_DATA_SO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_QUERY_DATA_SO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_SO_STATISTICS").field("NumPrimitivesWritten", &self.NumPrimitivesWritten).field("PrimitivesStorageNeeded", &self.PrimitivesStorageNeeded).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_QUERY_DATA_SO_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_SO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_QUERY_DATA_SO_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DATA_SO_STATISTICS {}
impl ::core::default::Default for D3D10_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_TIMESTAMP_DISJOINT").field("Frequency", &self.Frequency).field("Disjoint", &self.Disjoint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_QUERY_DATA_TIMESTAMP_DISJOINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_QUERY_DESC {
    pub Query: D3D10_QUERY,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_QUERY_DESC {}
impl ::core::clone::Clone for D3D10_QUERY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_QUERY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DESC").field("Query", &self.Query).field("MiscFlags", &self.MiscFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_QUERY_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_QUERY_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DESC {}
impl ::core::default::Default for D3D10_QUERY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_QUERY_MISC_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_QUERY_MISC_PREDICATEHINT: D3D10_QUERY_MISC_FLAG = D3D10_QUERY_MISC_FLAG(1i32);
impl ::core::marker::Copy for D3D10_QUERY_MISC_FLAG {}
impl ::core::clone::Clone for D3D10_QUERY_MISC_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_QUERY_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_QUERY_MISC_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_QUERY_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_QUERY_MISC_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_RAISE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D10_RAISE_FLAG = D3D10_RAISE_FLAG(1i32);
impl ::core::marker::Copy for D3D10_RAISE_FLAG {}
impl ::core::clone::Clone for D3D10_RAISE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_RAISE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_RAISE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_RAISE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RAISE_FLAG").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_RASTERIZER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_RASTERIZER_DESC")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("ScissorEnable", &self.ScissorEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_RASTERIZER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_RASTERIZER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_RASTERIZER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REGKEY_PATH: &str = "Software\\Microsoft\\Direct3D";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_RENDER_TARGET_BLEND_DESC1").field("BlendEnable", &self.BlendEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_RENDER_TARGET_BLEND_DESC1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_RENDER_TARGET_BLEND_DESC1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_RTV_DIMENSION,
    pub Anonymous: D3D10_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_RENDER_TARGET_VIEW_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_RENDER_TARGET_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_RENDER_TARGET_VIEW_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D10_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_RTV,
    pub Texture1D: D3D10_TEX1D_RTV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D10_TEX2D_RTV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D10_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D10_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_RENDER_TARGET_VIEW_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_MIP_LEVELS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURE1D_U_DIMENSION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_REQ_TEXTURECUBE_DIMENSION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_RESOURCE_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_DIMENSION_BUFFER: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(4i32);
impl ::core::marker::Copy for D3D10_RESOURCE_DIMENSION {}
impl ::core::clone::Clone for D3D10_RESOURCE_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_RESOURCE_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_RESOURCE_DIMENSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_RESOURCE_MISC_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_MISC_SHARED: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(32i32);
impl ::core::marker::Copy for D3D10_RESOURCE_MISC_FLAG {}
impl ::core::clone::Clone for D3D10_RESOURCE_MISC_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_RESOURCE_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_RESOURCE_MISC_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_RESOURCE_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_MISC_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_RTV_DIMENSION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_UNKNOWN: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_BUFFER: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE1D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE2D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_RTV_DIMENSION_TEXTURE3D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(8i32);
impl ::core::marker::Copy for D3D10_RTV_DIMENSION {}
impl ::core::clone::Clone for D3D10_RTV_DIMENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_RTV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_RTV_DIMENSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_RTV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RTV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SAMPLER_DESC {
    pub Filter: D3D10_FILTER,
    pub AddressU: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D10_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D10_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl ::core::marker::Copy for D3D10_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D10_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SAMPLER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SAMPLER_DESC").field("Filter", &self.Filter).field("AddressU", &self.AddressU).field("AddressV", &self.AddressV).field("AddressW", &self.AddressW).field("MipLODBias", &self.MipLODBias).field("MaxAnisotropy", &self.MaxAnisotropy).field("ComparisonFunc", &self.ComparisonFunc).field("BorderColor", &self.BorderColor).field("MinLOD", &self.MinLOD).field("MaxLOD", &self.MaxLOD).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SAMPLER_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SAMPLER_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SAMPLER_DESC {}
impl ::core::default::Default for D3D10_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SDK_VERSION: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_BUFFER_DESC").field("Name", &self.Name).field("Type", &self.Type).field("Variables", &self.Variables).field("Size", &self.Size).field("uFlags", &self.uFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_BUFFER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_BUFFER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_FILE_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_FILE_INFO").field("FileName", &self.FileName).field("FileNameLen", &self.FileNameLen).field("FileData", &self.FileData).field("FileLen", &self.FileLen).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_FILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_FILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_FILE_INFO {}
impl ::core::default::Default for D3D10_SHADER_DEBUG_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_DEBUG_INFO {
    pub Size: u32,
    pub Creator: u32,
    pub EntrypointName: u32,
    pub ShaderTarget: u32,
    pub CompileFlags: u32,
    pub Files: u32,
    pub FileInfo: u32,
    pub Instructions: u32,
    pub InstructionInfo: u32,
    pub Variables: u32,
    pub VariableInfo: u32,
    pub InputVariables: u32,
    pub InputVariableInfo: u32,
    pub Tokens: u32,
    pub TokenInfo: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub ScopeVariables: u32,
    pub ScopeVariableInfo: u32,
    pub UintOffset: u32,
    pub StringOffset: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INFO")
            .field("Size", &self.Size)
            .field("Creator", &self.Creator)
            .field("EntrypointName", &self.EntrypointName)
            .field("ShaderTarget", &self.ShaderTarget)
            .field("CompileFlags", &self.CompileFlags)
            .field("Files", &self.Files)
            .field("FileInfo", &self.FileInfo)
            .field("Instructions", &self.Instructions)
            .field("InstructionInfo", &self.InstructionInfo)
            .field("Variables", &self.Variables)
            .field("VariableInfo", &self.VariableInfo)
            .field("InputVariables", &self.InputVariables)
            .field("InputVariableInfo", &self.InputVariableInfo)
            .field("Tokens", &self.Tokens)
            .field("TokenInfo", &self.TokenInfo)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .field("ScopeVariables", &self.ScopeVariables)
            .field("ScopeVariableInfo", &self.ScopeVariableInfo)
            .field("UintOffset", &self.UintOffset)
            .field("StringOffset", &self.StringOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INFO {}
impl ::core::default::Default for D3D10_SHADER_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INPUT_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INPUT_INFO").field("Var", &self.Var).field("InitialRegisterSet", &self.InitialRegisterSet).field("InitialBank", &self.InitialBank).field("InitialRegister", &self.InitialRegister).field("InitialComponent", &self.InitialComponent).field("InitialValue", &self.InitialValue).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_INPUT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_INPUT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INPUT_INFO {}
impl ::core::default::Default for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_INST_INFO {
    pub Id: u32,
    pub Opcode: u32,
    pub uOutputs: u32,
    pub pOutputs: [D3D10_SHADER_DEBUG_OUTPUTREG_INFO; 2],
    pub TokenId: u32,
    pub NestingLevel: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub AccessedVars: u32,
    pub AccessedVarsInfo: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INST_INFO").field("Id", &self.Id).field("Opcode", &self.Opcode).field("uOutputs", &self.uOutputs).field("pOutputs", &self.pOutputs).field("TokenId", &self.TokenId).field("NestingLevel", &self.NestingLevel).field("Scopes", &self.Scopes).field("ScopeInfo", &self.ScopeInfo).field("AccessedVars", &self.AccessedVars).field("AccessedVarsInfo", &self.AccessedVarsInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_INST_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_INST_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_INST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    pub OutputRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub OutputReg: u32,
    pub TempArrayReg: u32,
    pub OutputComponents: [u32; 4],
    pub OutputVars: [D3D10_SHADER_DEBUG_OUTPUTVAR; 4],
    pub IndexReg: u32,
    pub IndexComp: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_OUTPUTREG_INFO").field("OutputRegisterSet", &self.OutputRegisterSet).field("OutputReg", &self.OutputReg).field("TempArrayReg", &self.TempArrayReg).field("OutputComponents", &self.OutputComponents).field("OutputVars", &self.OutputVars).field("IndexReg", &self.IndexReg).field("IndexComp", &self.IndexComp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_OUTPUTREG_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: super::super::Foundation::BOOL,
    pub bInfPossible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_OUTPUTVAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_OUTPUTVAR").field("Var", &self.Var).field("uValueMin", &self.uValueMin).field("uValueMax", &self.uValueMax).field("iValueMin", &self.iValueMin).field("iValueMax", &self.iValueMax).field("fValueMin", &self.fValueMin).field("fValueMax", &self.fValueMax).field("bNaNPossible", &self.bNaNPossible).field("bInfPossible", &self.bInfPossible).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_OUTPUTVAR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_OUTPUTVAR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_OUTPUTVAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_SHADER_DEBUG_REGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_INPUT: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_OUTPUT: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_CBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_TBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_TEMP: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_TEMPARRAY: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_TEXTURE: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_SAMPLER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_LITERAL: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_UNUSED: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D11_SHADER_DEBUG_REG_UAV: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_REG_FORCE_DWORD: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(2147483647i32);
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_REGTYPE {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_REGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_REGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_REGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_REGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_REGTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_SHADER_DEBUG_SCOPETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_SCOPE_FORCE_DWORD: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(2147483647i32);
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_SCOPETYPE {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_SCOPETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_SCOPETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    pub TokenId: u32,
    pub VarType: D3D10_SHADER_DEBUG_VARTYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub StructMemberScope: u32,
    pub uArrayIndices: u32,
    pub ArrayElements: u32,
    pub ArrayStrides: u32,
    pub uVariables: u32,
    pub uFirstVariable: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_SCOPEVAR_INFO").field("TokenId", &self.TokenId).field("VarType", &self.VarType).field("Class", &self.Class).field("Rows", &self.Rows).field("Columns", &self.Columns).field("StructMemberScope", &self.StructMemberScope).field("uArrayIndices", &self.uArrayIndices).field("ArrayElements", &self.ArrayElements).field("ArrayStrides", &self.ArrayStrides).field("uVariables", &self.uVariables).field("uFirstVariable", &self.uFirstVariable).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_SCOPEVAR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    pub ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    pub Name: u32,
    pub uNameLen: u32,
    pub uVariables: u32,
    pub VariableData: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_SCOPE_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_SCOPE_INFO").field("ScopeType", &self.ScopeType).field("Name", &self.Name).field("uNameLen", &self.uNameLen).field("uVariables", &self.uVariables).field("VariableData", &self.VariableData).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_SCOPE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_SCOPE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_SCOPE_INFO {}
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    pub File: u32,
    pub Line: u32,
    pub Column: u32,
    pub TokenLength: u32,
    pub TokenId: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_TOKEN_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_TOKEN_INFO").field("File", &self.File).field("Line", &self.Line).field("Column", &self.Column).field("TokenLength", &self.TokenLength).field("TokenId", &self.TokenId).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_TOKEN_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_TOKEN_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_TOKEN_INFO {}
impl ::core::default::Default for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_SHADER_DEBUG_VARTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_DEBUG_VAR_FORCE_DWORD: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(2147483647i32);
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_VARTYPE {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_VARTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_VARTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_VARTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_VARTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_VARTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_VAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_VAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_VAR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_VAR_INFO").field("TokenId", &self.TokenId).field("Type", &self.Type).field("Register", &self.Register).field("Component", &self.Component).field("ScopeVar", &self.ScopeVar).field("ScopeVarOffset", &self.ScopeVarOffset).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DEBUG_VAR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_VAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DEBUG_VAR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_VAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_VAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: ::windows::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DESC")
            .field("Version", &self.Version)
            .field("Creator", &self.Creator)
            .field("Flags", &self.Flags)
            .field("ConstantBuffers", &self.ConstantBuffers)
            .field("BoundResources", &self.BoundResources)
            .field("InputParameters", &self.InputParameters)
            .field("OutputParameters", &self.OutputParameters)
            .field("InstructionCount", &self.InstructionCount)
            .field("TempRegisterCount", &self.TempRegisterCount)
            .field("TempArrayCount", &self.TempArrayCount)
            .field("DefCount", &self.DefCount)
            .field("DclCount", &self.DclCount)
            .field("TextureNormalInstructions", &self.TextureNormalInstructions)
            .field("TextureLoadInstructions", &self.TextureLoadInstructions)
            .field("TextureCompInstructions", &self.TextureCompInstructions)
            .field("TextureBiasInstructions", &self.TextureBiasInstructions)
            .field("TextureGradientInstructions", &self.TextureGradientInstructions)
            .field("FloatInstructionCount", &self.FloatInstructionCount)
            .field("IntInstructionCount", &self.IntInstructionCount)
            .field("UintInstructionCount", &self.UintInstructionCount)
            .field("StaticFlowControlCount", &self.StaticFlowControlCount)
            .field("DynamicFlowControlCount", &self.DynamicFlowControlCount)
            .field("MacroInstructionCount", &self.MacroInstructionCount)
            .field("ArrayInstructionCount", &self.ArrayInstructionCount)
            .field("CutInstructionCount", &self.CutInstructionCount)
            .field("EmitInstructionCount", &self.EmitInstructionCount)
            .field("GSOutputTopology", &self.GSOutputTopology)
            .field("GSMaxOutputVertexCount", &self.GSMaxOutputVertexCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_INPUT_BIND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_INPUT_BIND_DESC").field("Name", &self.Name).field("Type", &self.Type).field("BindPoint", &self.BindPoint).field("BindCount", &self.BindCount).field("uFlags", &self.uFlags).field("ReturnType", &self.ReturnType).field("Dimension", &self.Dimension).field("NumSamples", &self.NumSamples).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_INPUT_BIND_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_INPUT_BIND_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_INPUT_BIND_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_NO_PRESHADER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_PARTIAL_PRECISION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_RESOURCE_VIEW_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_RESOURCE_VIEW_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D10_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_RESOURCE_VIEW_DESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_RESOURCE_VIEW_DESC1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D10_SHADER_RESOURCE_VIEW_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
    pub TextureCubeArray: D3D10_TEXCUBE_ARRAY_SRV1,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_RESOURCE_VIEW_DESC1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_TYPE_DESC").field("Class", &self.Class).field("Type", &self.Type).field("Rows", &self.Rows).field("Columns", &self.Columns).field("Elements", &self.Elements).field("Members", &self.Members).field("Offset", &self.Offset).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_TYPE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_TYPE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: ::windows::core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D10_SHADER_VARIABLE_DESC {}
impl ::core::clone::Clone for D3D10_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_VARIABLE_DESC").field("Name", &self.Name).field("StartOffset", &self.StartOffset).field("Size", &self.Size).field("uFlags", &self.uFlags).field("DefaultValue", &self.DefaultValue).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SHADER_VARIABLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SHADER_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_VARIABLE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_VARIABLE_DESC {}
impl ::core::default::Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SIGNATURE_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SIGNATURE_PARAMETER_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Register", &self.Register).field("SystemValueType", &self.SystemValueType).field("ComponentType", &self.ComponentType).field("Mask", &self.Mask).field("ReadWriteMask", &self.ReadWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for D3D10_SIGNATURE_PARAMETER_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SIGNATURE_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SIGNATURE_PARAMETER_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: ::windows::core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl ::core::marker::Copy for D3D10_SO_DECLARATION_ENTRY {}
impl ::core::clone::Clone for D3D10_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SO_DECLARATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SO_DECLARATION_ENTRY").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("StartComponent", &self.StartComponent).field("ComponentCount", &self.ComponentCount).field("OutputSlot", &self.OutputSlot).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SO_DECLARATION_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SO_DECLARATION_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SO_DECLARATION_ENTRY {}
impl ::core::default::Default for D3D10_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_GAMMA: f32 = 2.2f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-2i32);
impl ::core::marker::Copy for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {}
impl ::core::clone::Clone for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_STATE_BLOCK_MASK {
    pub VS: u8,
    pub VSSamplers: [u8; 2],
    pub VSShaderResources: [u8; 16],
    pub VSConstantBuffers: [u8; 2],
    pub GS: u8,
    pub GSSamplers: [u8; 2],
    pub GSShaderResources: [u8; 16],
    pub GSConstantBuffers: [u8; 2],
    pub PS: u8,
    pub PSSamplers: [u8; 2],
    pub PSShaderResources: [u8; 16],
    pub PSConstantBuffers: [u8; 2],
    pub IAVertexBuffers: [u8; 2],
    pub IAIndexBuffer: u8,
    pub IAInputLayout: u8,
    pub IAPrimitiveTopology: u8,
    pub OMRenderTargets: u8,
    pub OMDepthStencilState: u8,
    pub OMBlendState: u8,
    pub RSViewports: u8,
    pub RSScissorRects: u8,
    pub RSRasterizerState: u8,
    pub SOBuffers: u8,
    pub Predication: u8,
}
impl ::core::marker::Copy for D3D10_STATE_BLOCK_MASK {}
impl ::core::clone::Clone for D3D10_STATE_BLOCK_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_STATE_BLOCK_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_STATE_BLOCK_MASK")
            .field("VS", &self.VS)
            .field("VSSamplers", &self.VSSamplers)
            .field("VSShaderResources", &self.VSShaderResources)
            .field("VSConstantBuffers", &self.VSConstantBuffers)
            .field("GS", &self.GS)
            .field("GSSamplers", &self.GSSamplers)
            .field("GSShaderResources", &self.GSShaderResources)
            .field("GSConstantBuffers", &self.GSConstantBuffers)
            .field("PS", &self.PS)
            .field("PSSamplers", &self.PSSamplers)
            .field("PSShaderResources", &self.PSShaderResources)
            .field("PSConstantBuffers", &self.PSConstantBuffers)
            .field("IAVertexBuffers", &self.IAVertexBuffers)
            .field("IAIndexBuffer", &self.IAIndexBuffer)
            .field("IAInputLayout", &self.IAInputLayout)
            .field("IAPrimitiveTopology", &self.IAPrimitiveTopology)
            .field("OMRenderTargets", &self.OMRenderTargets)
            .field("OMDepthStencilState", &self.OMDepthStencilState)
            .field("OMBlendState", &self.OMBlendState)
            .field("RSViewports", &self.RSViewports)
            .field("RSScissorRects", &self.RSScissorRects)
            .field("RSRasterizerState", &self.RSRasterizerState)
            .field("SOBuffers", &self.SOBuffers)
            .field("Predication", &self.Predication)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_STATE_BLOCK_MASK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_STATE_BLOCK_MASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_STATE_BLOCK_MASK>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_STATE_BLOCK_MASK {}
impl ::core::default::Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_STENCIL_OP(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_KEEP: D3D10_STENCIL_OP = D3D10_STENCIL_OP(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_ZERO: D3D10_STENCIL_OP = D3D10_STENCIL_OP(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_REPLACE: D3D10_STENCIL_OP = D3D10_STENCIL_OP(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_INCR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_DECR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_INVERT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_INCR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_STENCIL_OP_DECR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(8i32);
impl ::core::marker::Copy for D3D10_STENCIL_OP {}
impl ::core::clone::Clone for D3D10_STENCIL_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_STENCIL_OP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_STENCIL_OP {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_STENCIL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_STENCIL_OP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *const ::core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl ::core::marker::Copy for D3D10_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D10_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_SUBRESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SUBRESOURCE_DATA").field("pSysMem", &self.pSysMem).field("SysMemPitch", &self.SysMemPitch).field("SysMemSlicePitch", &self.SysMemSlicePitch).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_SUBRESOURCE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_SUBRESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SUBRESOURCE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_SUBRESOURCE_DATA {}
impl ::core::default::Default for D3D10_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: ::windows::core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
impl ::core::marker::Copy for D3D10_TECHNIQUE_DESC {}
impl ::core::clone::Clone for D3D10_TECHNIQUE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TECHNIQUE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TECHNIQUE_DESC").field("Name", &self.Name).field("Passes", &self.Passes).field("Annotations", &self.Annotations).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TECHNIQUE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TECHNIQUE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TECHNIQUE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TECHNIQUE_DESC {}
impl ::core::default::Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_ARRAY_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_ARRAY_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_DSV {}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_ARRAY_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_ARRAY_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_RTV {}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_ARRAY_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_ARRAY_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_SRV {}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_DSV {}
impl ::core::clone::Clone for D3D10_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_DSV {}
impl ::core::default::Default for D3D10_TEX1D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_RTV {}
impl ::core::clone::Clone for D3D10_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_RTV {}
impl ::core::default::Default for D3D10_TEX1D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_SRV {}
impl ::core::clone::Clone for D3D10_TEX1D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX1D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX1D_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX1D_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_SRV {}
impl ::core::default::Default for D3D10_TEX1D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_DSV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_ARRAY_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_ARRAY_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_DSV {}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_RTV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_ARRAY_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_ARRAY_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_RTV {}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_SRV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_ARRAY_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_ARRAY_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_SRV {}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_DSV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_DSV {}
impl ::core::default::Default for D3D10_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_RTV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_RTV {}
impl ::core::default::Default for D3D10_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2DMS_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_SRV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2DMS_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2DMS_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_SRV {}
impl ::core::default::Default for D3D10_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_ARRAY_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_ARRAY_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_DSV {}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_ARRAY_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_ARRAY_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_RTV {}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_ARRAY_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_ARRAY_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_SRV {}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_DSV {}
impl ::core::clone::Clone for D3D10_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_DSV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_DSV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_DSV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_DSV {}
impl ::core::default::Default for D3D10_TEX2D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_RTV {}
impl ::core::clone::Clone for D3D10_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_RTV {}
impl ::core::default::Default for D3D10_TEX2D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_SRV {}
impl ::core::clone::Clone for D3D10_TEX2D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX2D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX2D_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX2D_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_SRV {}
impl ::core::default::Default for D3D10_TEX2D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D10_TEX3D_RTV {}
impl ::core::clone::Clone for D3D10_TEX3D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX3D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX3D_RTV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX3D_RTV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX3D_RTV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX3D_RTV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX3D_RTV {}
impl ::core::default::Default for D3D10_TEX3D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX3D_SRV {}
impl ::core::clone::Clone for D3D10_TEX3D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEX3D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX3D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEX3D_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEX3D_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEX3D_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEX3D_SRV {}
impl ::core::default::Default for D3D10_TEX3D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
impl ::core::marker::Copy for D3D10_TEXCUBE_ARRAY_SRV1 {}
impl ::core::clone::Clone for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXCUBE_ARRAY_SRV1").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("First2DArrayFace", &self.First2DArrayFace).field("NumCubes", &self.NumCubes).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEXCUBE_ARRAY_SRV1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEXCUBE_ARRAY_SRV1>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEXCUBE_ARRAY_SRV1 {}
impl ::core::default::Default for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEXCUBE_SRV {}
impl ::core::clone::Clone for D3D10_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_TEXCUBE_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXCUBE_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEXCUBE_SRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_TEXCUBE_SRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEXCUBE_SRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_TEXCUBE_SRV {}
impl ::core::default::Default for D3D10_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE1D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE1D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE1D_DESC").field("Width", &self.Width).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_TEXTURE1D_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE1D_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEXTURE1D_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE1D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE2D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE2D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE2D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_TEXTURE2D_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE2D_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEXTURE2D_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE2D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE3D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE3D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE3D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for D3D10_TEXTURE3D_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE3D_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TEXTURE3D_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE3D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_TEXTURECUBE_FACE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_X: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_X: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Y: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Y: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Z: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Z: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(5i32);
impl ::core::marker::Copy for D3D10_TEXTURECUBE_FACE {}
impl ::core::clone::Clone for D3D10_TEXTURECUBE_FACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_TEXTURECUBE_FACE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEXTURECUBE_FACE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_TEXTURECUBE_FACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURECUBE_FACE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_TEXTURE_ADDRESS_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURE_ADDRESS_WRAP: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURE_ADDRESS_MIRROR: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURE_ADDRESS_CLAMP: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURE_ADDRESS_BORDER: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXTURE_ADDRESS_MIRROR_ONCE: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(5i32);
impl ::core::marker::Copy for D3D10_TEXTURE_ADDRESS_MODE {}
impl ::core::clone::Clone for D3D10_TEXTURE_ADDRESS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_TEXTURE_ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_TEXTURE_ADDRESS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_UNMUTE_SEVERITY_INFO: &str = "Unmute_SEVERITY_INFO";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D10_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_USAGE_DEFAULT: D3D10_USAGE = D3D10_USAGE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_USAGE_IMMUTABLE: D3D10_USAGE = D3D10_USAGE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_USAGE_DYNAMIC: D3D10_USAGE = D3D10_USAGE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_USAGE_STAGING: D3D10_USAGE = D3D10_USAGE(3i32);
impl ::core::marker::Copy for D3D10_USAGE {}
impl ::core::clone::Clone for D3D10_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D10_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D10_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D10_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_USAGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub struct D3D10_VIEWPORT {
    pub TopLeftX: i32,
    pub TopLeftY: i32,
    pub Width: u32,
    pub Height: u32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl ::core::marker::Copy for D3D10_VIEWPORT {}
impl ::core::clone::Clone for D3D10_VIEWPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D10_VIEWPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_VIEWPORT").field("TopLeftX", &self.TopLeftX).field("TopLeftY", &self.TopLeftY).field("Width", &self.Width).field("Height", &self.Height).field("MinDepth", &self.MinDepth).field("MaxDepth", &self.MaxDepth).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D10_VIEWPORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D10_VIEWPORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_VIEWPORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D10_VIEWPORT {}
impl ::core::default::Default for D3D10_VIEWPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VIEWPORT_BOUNDS_MAX: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VIEWPORT_BOUNDS_MIN: i32 = -16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_INPUT_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_VS_OUTPUT_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_MAJOR_VERSION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_SPEC_DATE_DAY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_SPEC_DATE_MONTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_SPEC_DATE_YEAR: u32 = 2006u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const D3D_SPEC_VERSION: f64 = 1.050005f64;
pub const DXGI_DEBUG_D3D10: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243b4c52_3606_4d3a_99d7_a7e7b33ed706);
pub const GUID_DeviceType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Asynchronous(::windows::core::IUnknown);
impl ID3D10Asynchronous {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).Begin)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).End)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetDataSize)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10Asynchronous> for ::windows::core::IUnknown {
    fn from(value: ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Asynchronous> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Asynchronous> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Asynchronous) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Asynchronous> for ID3D10DeviceChild {
    fn from(value: ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Asynchronous> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Asynchronous> for ID3D10DeviceChild {
    fn from(value: &ID3D10Asynchronous) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Asynchronous {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Asynchronous {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Asynchronous {}
impl ::core::fmt::Debug for ID3D10Asynchronous {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Asynchronous").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Asynchronous {}
unsafe impl ::core::marker::Sync for ID3D10Asynchronous {}
unsafe impl ::windows::core::Interface for ID3D10Asynchronous {
    type Vtable = ID3D10Asynchronous_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0d_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Asynchronous_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10BlendState(::windows::core::IUnknown);
impl ID3D10BlendState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10BlendState> for ::windows::core::IUnknown {
    fn from(value: ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10BlendState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10BlendState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10BlendState> for ID3D10DeviceChild {
    fn from(value: ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10BlendState> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState> for ID3D10DeviceChild {
    fn from(value: &ID3D10BlendState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10BlendState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10BlendState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10BlendState {}
impl ::core::fmt::Debug for ID3D10BlendState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10BlendState").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10BlendState {}
unsafe impl ::core::marker::Sync for ID3D10BlendState {}
unsafe impl ::windows::core::Interface for ID3D10BlendState {
    type Vtable = ID3D10BlendState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedad8d19_8a35_4d6d_8566_2ea276cde161);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10BlendState1(::windows::core::IUnknown);
impl ID3D10BlendState1 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1) {
        (::windows::core::Interface::vtable(self).GetDesc1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ::windows::core::IUnknown {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10BlendState1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ID3D10DeviceChild {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10BlendState1> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ID3D10DeviceChild {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ID3D10BlendState {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10BlendState1> for &'a ID3D10BlendState {
    fn from(value: &'a ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ID3D10BlendState {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10BlendState1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10BlendState1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10BlendState1 {}
impl ::core::fmt::Debug for ID3D10BlendState1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10BlendState1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10BlendState1 {}
unsafe impl ::core::marker::Sync for ID3D10BlendState1 {}
unsafe impl ::windows::core::Interface for ID3D10BlendState1 {
    type Vtable = ID3D10BlendState1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedad8d99_8a35_4d6d_8566_2ea276cde161);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState1_Vtbl {
    pub base__: ID3D10BlendState_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Buffer(::windows::core::IUnknown);
impl ID3D10Buffer {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).base__.SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetEvictionPriority)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), maptype, mapflags, ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn Unmap(&self) {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Buffer> for ::windows::core::IUnknown {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Buffer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Buffer> for ID3D10DeviceChild {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Buffer> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ID3D10DeviceChild {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Buffer> for ID3D10Resource {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Buffer> for &'a ID3D10Resource {
    fn from(value: &'a ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ID3D10Resource {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Buffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Buffer {}
impl ::core::fmt::Debug for ID3D10Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Buffer").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Buffer {}
unsafe impl ::core::marker::Sync for ID3D10Buffer {}
unsafe impl ::windows::core::Interface for ID3D10Buffer {
    type Vtable = ID3D10Buffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c02_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Buffer_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Counter(::windows::core::IUnknown);
impl ID3D10Counter {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).base__.Begin)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).base__.End)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetDataSize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Counter> for ::windows::core::IUnknown {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Counter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Counter> for ID3D10DeviceChild {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Counter> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ID3D10DeviceChild {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Counter> for ID3D10Asynchronous {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Counter> for &'a ID3D10Asynchronous {
    fn from(value: &'a ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ID3D10Asynchronous {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Counter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Counter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Counter {}
impl ::core::fmt::Debug for ID3D10Counter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Counter").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Counter {}
unsafe impl ::core::marker::Sync for ID3D10Counter {}
unsafe impl ::windows::core::Interface for ID3D10Counter {
    type Vtable = ID3D10Counter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c11_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Counter_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Debug(::windows::core::IUnknown);
impl ID3D10Debug {
    pub unsafe fn SetFeatureMask(&self, mask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFeatureMask)(::windows::core::Interface::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetFeatureMask)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPresentPerRenderOpDelay)(::windows::core::Interface::as_raw(self), milliseconds).ok()
    }
    pub unsafe fn GetPresentPerRenderOpDelay(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetPresentPerRenderOpDelay)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, P0>(&self, pswapchain: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dxgi::IDXGISwapChain>>,
    {
        (::windows::core::Interface::vtable(self).SetSwapChain)(::windows::core::Interface::as_raw(self), pswapchain.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSwapChain(&self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSwapChain)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Dxgi::IDXGISwapChain>(result__)
    }
    pub unsafe fn Validate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ID3D10Debug> for ::windows::core::IUnknown {
    fn from(value: ID3D10Debug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Debug> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Debug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Debug> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Debug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Debug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Debug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Debug {}
impl ::core::fmt::Debug for ID3D10Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Debug").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Debug {}
unsafe impl ::core::marker::Sync for ID3D10Debug {}
unsafe impl ::windows::core::Interface for ID3D10Debug {
    type Vtable = ID3D10Debug_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e01_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Debug_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub SetPresentPerRenderOpDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT,
    pub GetPresentPerRenderOpDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pswapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSwapChain: usize,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10DepthStencilState(::windows::core::IUnknown);
impl ID3D10DepthStencilState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10DepthStencilState> for ::windows::core::IUnknown {
    fn from(value: ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DepthStencilState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DepthStencilState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10DepthStencilState> for ID3D10DeviceChild {
    fn from(value: ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DepthStencilState> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilState> for ID3D10DeviceChild {
    fn from(value: &ID3D10DepthStencilState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10DepthStencilState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10DepthStencilState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DepthStencilState {}
impl ::core::fmt::Debug for ID3D10DepthStencilState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DepthStencilState").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10DepthStencilState {}
unsafe impl ::core::marker::Sync for ID3D10DepthStencilState {}
unsafe impl ::windows::core::Interface for ID3D10DepthStencilState {
    type Vtable = ID3D10DepthStencilState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b4b1cc8_a4ad_41f8_8322_ca86fc3ec675);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10DepthStencilView(::windows::core::IUnknown);
impl ID3D10DepthStencilView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).base__.GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresource))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ::windows::core::IUnknown {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DepthStencilView> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ID3D10DeviceChild {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DepthStencilView> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ID3D10DeviceChild {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ID3D10View {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DepthStencilView> for &'a ID3D10View {
    fn from(value: &'a ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ID3D10View {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10DepthStencilView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10DepthStencilView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DepthStencilView {}
impl ::core::fmt::Debug for ID3D10DepthStencilView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DepthStencilView").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10DepthStencilView {}
unsafe impl ::core::marker::Sync for ID3D10DepthStencilView {}
unsafe impl ::windows::core::Interface for ID3D10DepthStencilView {
    type Vtable = ID3D10DepthStencilView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c09_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Device(::windows::core::IUnknown);
impl ID3D10Device {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).VSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).PSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn PSSetShader<'a, P0>(&self, ppixelshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10PixelShader>>,
    {
        (::windows::core::Interface::vtable(self).PSSetShader)(::windows::core::Interface::as_raw(self), ppixelshader.into().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).PSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn VSSetShader<'a, P0>(&self, pvertexshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10VertexShader>>,
    {
        (::windows::core::Interface::vtable(self).VSSetShader)(::windows::core::Interface::as_raw(self), pvertexshader.into().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Interface::vtable(self).DrawIndexed)(::windows::core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Interface::vtable(self).Draw)(::windows::core::Interface::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).PSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn IASetInputLayout<'a, P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10InputLayout>>,
    {
        (::windows::core::Interface::vtable(self).IASetInputLayout)(::windows::core::Interface::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).IASetVertexBuffers)(::windows::core::Interface::as_raw(self), startslot, ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<'a, P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Buffer>>,
    {
        (::windows::core::Interface::vtable(self).IASetIndexBuffer)(::windows::core::Interface::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).DrawIndexedInstanced)(::windows::core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).DrawInstanced)(::windows::core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).GSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn GSSetShader<'a, P0>(&self, pshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10GeometryShader>>,
    {
        (::windows::core::Interface::vtable(self).GSSetShader)(::windows::core::Interface::as_raw(self), pshader.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).IASetPrimitiveTopology)(::windows::core::Interface::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).VSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).VSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<'a, P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPredication)(::windows::core::Interface::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).GSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).GSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn OMSetRenderTargets<'a, P0>(&self, pprendertargetviews: &[::core::option::Option<ID3D10RenderTargetView>], pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilView>>,
    {
        (::windows::core::Interface::vtable(self).OMSetRenderTargets)(::windows::core::Interface::as_raw(self), pprendertargetviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pprendertargetviews)), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetBlendState<'a, P0>(&self, pblendstate: P0, blendfactor: *const f32, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10BlendState>>,
    {
        (::windows::core::Interface::vtable(self).OMSetBlendState)(::windows::core::Interface::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<'a, P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilState>>,
    {
        (::windows::core::Interface::vtable(self).OMSetDepthStencilState)(::windows::core::Interface::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).SOSetTargets)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Interface::vtable(self).DrawAuto)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn RSSetState<'a, P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10RasterizerState>>,
    {
        (::windows::core::Interface::vtable(self).RSSetState)(::windows::core::Interface::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D10_VIEWPORT]) {
        (::windows::core::Interface::vtable(self).RSSetViewports)(::windows::core::Interface::as_raw(self), pviewports.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pviewports)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Interface::vtable(self).RSSetScissorRects)(::windows::core::Interface::as_raw(self), prects.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(prects)))
    }
    pub unsafe fn CopySubresourceRegion<'a, P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: *const D3D10_BOX)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).CopySubresourceRegion)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox))
    }
    pub unsafe fn CopyResource<'a, P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).CopyResource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<'a, P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).UpdateSubresource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox), ::core::mem::transmute(psrcdata), srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ClearRenderTargetView<'a, P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10RenderTargetView>>,
    {
        (::windows::core::Interface::vtable(self).ClearRenderTargetView)(::windows::core::Interface::as_raw(self), prendertargetview.into().abi(), ::core::mem::transmute(colorrgba))
    }
    pub unsafe fn ClearDepthStencilView<'a, P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilView>>,
    {
        (::windows::core::Interface::vtable(self).ClearDepthStencilView)(::windows::core::Interface::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<'a, P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10ShaderResourceView>>,
    {
        (::windows::core::Interface::vtable(self).GenerateMips)(::windows::core::Interface::as_raw(self), pshaderresourceview.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<'a, P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).ResolveSubresource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).VSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).PSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>) {
        (::windows::core::Interface::vtable(self).PSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pppixelshader))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).PSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>) {
        (::windows::core::Interface::vtable(self).VSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppvertexshader))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).PSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>) {
        (::windows::core::Interface::vtable(self).IAGetInputLayout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppinputlayout))
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).IAGetVertexBuffers)(::windows::core::Interface::as_raw(self), startslot, ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
        (::windows::core::Interface::vtable(self).IAGetIndexBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindexbuffer), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).GSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) {
        (::windows::core::Interface::vtable(self).GSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppgeometryshader))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).IAGetPrimitiveTopology)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptopology))
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).VSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).VSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL) {
        (::windows::core::Interface::vtable(self).GetPredication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pppredicate), ::core::mem::transmute(ppredicatevalue))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).GSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).GSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: &mut [::core::option::Option<ID3D10RenderTargetView>], ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>) {
        (::windows::core::Interface::vtable(self).OMGetRenderTargets)(::windows::core::Interface::as_raw(self), pprendertargetviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pprendertargetviews)), ::core::mem::transmute(ppdepthstencilview))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32) {
        (::windows::core::Interface::vtable(self).OMGetBlendState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppblendstate), ::core::mem::transmute(blendfactor), ::core::mem::transmute(psamplemask))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32) {
        (::windows::core::Interface::vtable(self).OMGetDepthStencilState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdepthstencilstate), ::core::mem::transmute(pstencilref))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).SOGetTargets)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>) {
        (::windows::core::Interface::vtable(self).RSGetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprasterizerstate))
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).RSGetViewports)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).RSGetScissorRects)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceRemovedReason)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExceptionMode)(::windows::core::Interface::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetExceptionMode)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Interface::vtable(self).ClearState)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Buffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTexture1D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture1D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTexture2D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture2D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTexture3D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateShaderResourceView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateRenderTargetView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RenderTargetView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDepthStencilView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8]) -> ::windows::core::Result<ID3D10InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateInputLayout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pinputelementdescs)), pinputelementdescs.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecodewithinputsignature)), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10InputLayout>(result__)
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateVertexShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateGeometryShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: &[D3D10_SO_DECLARATION_ENTRY], outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psodeclaration)), psodeclaration.len() as _, outputstreamstride, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreatePixelShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10PixelShader>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlendState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10BlendState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDepthStencilState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdepthstencildesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateRasterizerState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prasterizerdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RasterizerState>(result__)
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateSamplerState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psamplerdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateQuery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pquerydesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Query>(result__)
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreatePredicate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppredicatedesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Predicate>(result__)
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateCounter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcounterdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Counter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CheckFormatSupport)(::windows::core::Interface::as_raw(self), format, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CheckMultisampleQualityLevels)(::windows::core::Interface::as_raw(self), format, samplecount, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO) {
        (::windows::core::Interface::vtable(self).CheckCounterInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcounterinfo))
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckCounter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(ptype), ::core::mem::transmute(pactivecounters), ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength)).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<'a, P0>(&self, hresource: P0, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).OpenSharedResource)(::windows::core::Interface::as_raw(self), hresource.into(), ::core::mem::transmute(returnedinterface), ::core::mem::transmute(ppresource)).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (::windows::core::Interface::vtable(self).SetTextFilterSize)(::windows::core::Interface::as_raw(self), width, height)
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32) {
        (::windows::core::Interface::vtable(self).GetTextFilterSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight))
    }
}
impl ::core::convert::From<ID3D10Device> for ::windows::core::IUnknown {
    fn from(value: ID3D10Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Device> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Device) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Device {}
impl ::core::fmt::Debug for ID3D10Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Device").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Device {}
unsafe impl ::core::marker::Sync for ID3D10Device {}
unsafe impl ::windows::core::Interface for ID3D10Device {
    type Vtable = ID3D10Device_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0f_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void),
    pub PSSetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelshader: *mut ::core::ffi::c_void),
    pub PSSetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvertexshader: *mut ::core::ffi::c_void),
    pub DrawIndexed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32),
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32),
    pub PSSetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputlayout: *mut ::core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut ::core::ffi::c_void, pstrides: *const u32, poffsets: *const u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IASetIndexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IASetIndexBuffer: usize,
    pub DrawIndexedInstanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32),
    pub DrawInstanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32),
    pub GSSetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IASetPrimitiveTopology: usize,
    pub VSSetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void),
    pub VSSetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub SetPredication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicate: *mut ::core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPredication: usize,
    pub GSSetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void),
    pub GSSetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void),
    pub OMSetBlendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstate: *mut ::core::ffi::c_void, blendfactor: *const f32, samplemask: u32),
    pub OMSetDepthStencilState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilstate: *mut ::core::ffi::c_void, stencilref: u32),
    pub SOSetTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut ::core::ffi::c_void, poffsets: *const u32),
    pub DrawAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub RSSetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerstate: *mut ::core::ffi::c_void),
    pub RSSetViewports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")]
    pub RSSetScissorRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    RSSetScissorRects: usize,
    pub CopySubresourceRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX),
    pub CopyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void),
    pub UpdateSubresource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32),
    pub ClearRenderTargetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertargetview: *mut ::core::ffi::c_void, colorrgba: *const f32),
    pub ClearDepthStencilView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8),
    pub GenerateMips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderresourceview: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresource: usize,
    pub VSGetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void),
    pub PSGetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppixelshader: *mut *mut ::core::ffi::c_void),
    pub PSGetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvertexshader: *mut *mut ::core::ffi::c_void),
    pub PSGetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void),
    pub IAGetInputLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinputlayout: *mut *mut ::core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut ::core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IAGetIndexBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: *mut *mut ::core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IAGetPrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IAGetPrimitiveTopology: usize,
    pub VSGetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void),
    pub VSGetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub GetPredication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppredicate: *mut *mut ::core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPredication: usize,
    pub GSGetShaderResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void),
    pub GSGetSamplers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut ::core::ffi::c_void, ppdepthstencilview: *mut *mut ::core::ffi::c_void),
    pub OMGetBlendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppblendstate: *mut *mut ::core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32),
    pub OMGetDepthStencilState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut *mut ::core::ffi::c_void, pstencilref: *mut u32),
    pub SOGetTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut ::core::ffi::c_void, poffsets: *mut u32),
    pub RSGetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut *mut ::core::ffi::c_void),
    pub RSGetViewports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")]
    pub RSGetScissorRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    RSGetScissorRects: usize,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetExceptionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub CreateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture1D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture1D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture2D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture2D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture3D: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateDepthStencilView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateDepthStencilView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateInputLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateInputLayout: usize,
    pub CreateVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateBlendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateBlendState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDepthStencilState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDepthStencilState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRasterizerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRasterizerState: usize,
    pub CreateSamplerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenSharedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenSharedResource: usize,
    pub SetTextFilterSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32),
    pub GetTextFilterSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Device1(::windows::core::IUnknown);
impl ID3D10Device1 {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.VSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.PSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn PSSetShader<'a, P0>(&self, ppixelshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10PixelShader>>,
    {
        (::windows::core::Interface::vtable(self).base__.PSSetShader)(::windows::core::Interface::as_raw(self), ppixelshader.into().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.PSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn VSSetShader<'a, P0>(&self, pvertexshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10VertexShader>>,
    {
        (::windows::core::Interface::vtable(self).base__.VSSetShader)(::windows::core::Interface::as_raw(self), pvertexshader.into().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Interface::vtable(self).base__.DrawIndexed)(::windows::core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Interface::vtable(self).base__.Draw)(::windows::core::Interface::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.PSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn IASetInputLayout<'a, P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10InputLayout>>,
    {
        (::windows::core::Interface::vtable(self).base__.IASetInputLayout)(::windows::core::Interface::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).base__.IASetVertexBuffers)(::windows::core::Interface::as_raw(self), startslot, ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<'a, P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Buffer>>,
    {
        (::windows::core::Interface::vtable(self).base__.IASetIndexBuffer)(::windows::core::Interface::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).base__.DrawIndexedInstanced)(::windows::core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).base__.DrawInstanced)(::windows::core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &[::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.GSSetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn GSSetShader<'a, P0>(&self, pshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10GeometryShader>>,
    {
        (::windows::core::Interface::vtable(self).base__.GSSetShader)(::windows::core::Interface::as_raw(self), pshader.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).base__.IASetPrimitiveTopology)(::windows::core::Interface::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.VSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.VSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<'a, P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPredication)(::windows::core::Interface::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: &[::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.GSSetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: &[::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.GSSetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn OMSetRenderTargets<'a, P0>(&self, pprendertargetviews: &[::core::option::Option<ID3D10RenderTargetView>], pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilView>>,
    {
        (::windows::core::Interface::vtable(self).base__.OMSetRenderTargets)(::windows::core::Interface::as_raw(self), pprendertargetviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pprendertargetviews)), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetBlendState<'a, P0>(&self, pblendstate: P0, blendfactor: *const f32, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10BlendState>>,
    {
        (::windows::core::Interface::vtable(self).base__.OMSetBlendState)(::windows::core::Interface::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<'a, P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilState>>,
    {
        (::windows::core::Interface::vtable(self).base__.OMSetDepthStencilState)(::windows::core::Interface::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).base__.SOSetTargets)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Interface::vtable(self).base__.DrawAuto)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn RSSetState<'a, P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10RasterizerState>>,
    {
        (::windows::core::Interface::vtable(self).base__.RSSetState)(::windows::core::Interface::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D10_VIEWPORT]) {
        (::windows::core::Interface::vtable(self).base__.RSSetViewports)(::windows::core::Interface::as_raw(self), pviewports.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pviewports)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Interface::vtable(self).base__.RSSetScissorRects)(::windows::core::Interface::as_raw(self), prects.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(prects)))
    }
    pub unsafe fn CopySubresourceRegion<'a, P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: *const D3D10_BOX)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopySubresourceRegion)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox))
    }
    pub unsafe fn CopyResource<'a, P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyResource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<'a, P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).base__.UpdateSubresource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox), ::core::mem::transmute(psrcdata), srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ClearRenderTargetView<'a, P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10RenderTargetView>>,
    {
        (::windows::core::Interface::vtable(self).base__.ClearRenderTargetView)(::windows::core::Interface::as_raw(self), prendertargetview.into().abi(), ::core::mem::transmute(colorrgba))
    }
    pub unsafe fn ClearDepthStencilView<'a, P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilView>>,
    {
        (::windows::core::Interface::vtable(self).base__.ClearDepthStencilView)(::windows::core::Interface::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<'a, P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10ShaderResourceView>>,
    {
        (::windows::core::Interface::vtable(self).base__.GenerateMips)(::windows::core::Interface::as_raw(self), pshaderresourceview.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<'a, P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        (::windows::core::Interface::vtable(self).base__.ResolveSubresource)(::windows::core::Interface::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.VSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.PSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>) {
        (::windows::core::Interface::vtable(self).base__.PSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pppixelshader))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.PSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>) {
        (::windows::core::Interface::vtable(self).base__.VSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppvertexshader))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.PSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>) {
        (::windows::core::Interface::vtable(self).base__.IAGetInputLayout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppinputlayout))
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.IAGetVertexBuffers)(::windows::core::Interface::as_raw(self), startslot, ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.IAGetIndexBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindexbuffer), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: &mut [::core::option::Option<ID3D10Buffer>]) {
        (::windows::core::Interface::vtable(self).base__.GSGetConstantBuffers)(::windows::core::Interface::as_raw(self), startslot, ppconstantbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppconstantbuffers)))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) {
        (::windows::core::Interface::vtable(self).base__.GSGetShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppgeometryshader))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).base__.IAGetPrimitiveTopology)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptopology))
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.VSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.VSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL) {
        (::windows::core::Interface::vtable(self).base__.GetPredication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pppredicate), ::core::mem::transmute(ppredicatevalue))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: &mut [::core::option::Option<ID3D10ShaderResourceView>]) {
        (::windows::core::Interface::vtable(self).base__.GSGetShaderResources)(::windows::core::Interface::as_raw(self), startslot, ppshaderresourceviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppshaderresourceviews)))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: &mut [::core::option::Option<ID3D10SamplerState>]) {
        (::windows::core::Interface::vtable(self).base__.GSGetSamplers)(::windows::core::Interface::as_raw(self), startslot, ppsamplers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppsamplers)))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: &mut [::core::option::Option<ID3D10RenderTargetView>], ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>) {
        (::windows::core::Interface::vtable(self).base__.OMGetRenderTargets)(::windows::core::Interface::as_raw(self), pprendertargetviews.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pprendertargetviews)), ::core::mem::transmute(ppdepthstencilview))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.OMGetBlendState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppblendstate), ::core::mem::transmute(blendfactor), ::core::mem::transmute(psamplemask))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.OMGetDepthStencilState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdepthstencilstate), ::core::mem::transmute(pstencilref))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.SOGetTargets)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>) {
        (::windows::core::Interface::vtable(self).base__.RSGetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprasterizerstate))
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).base__.RSGetViewports)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).base__.RSGetScissorRects)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetExceptionMode)(::windows::core::Interface::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetExceptionMode)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Interface::vtable(self).base__.ClearState)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Interface::vtable(self).base__.Flush)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Buffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateTexture1D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture1D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateTexture2D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture2D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateTexture3D)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Texture3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateShaderResourceView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateRenderTargetView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RenderTargetView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<'a, P0>(&self, presource: P0, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDepthStencilView)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilView>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8]) -> ::windows::core::Result<ID3D10InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateInputLayout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pinputelementdescs)), pinputelementdescs.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecodewithinputsignature)), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10InputLayout>(result__)
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateVertexShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateGeometryShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: &[D3D10_SO_DECLARATION_ENTRY], outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateGeometryShaderWithStreamOutput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(psodeclaration)), psodeclaration.len() as _, outputstreamstride, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8]) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreatePixelShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderbytecode)), pshaderbytecode.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10PixelShader>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBlendState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10BlendState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDepthStencilState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdepthstencildesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateRasterizerState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prasterizerdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RasterizerState>(result__)
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateSamplerState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psamplerdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateQuery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pquerydesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Query>(result__)
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreatePredicate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppredicatedesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Predicate>(result__)
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateCounter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcounterdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Counter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CheckFormatSupport)(::windows::core::Interface::as_raw(self), format, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CheckMultisampleQualityLevels)(::windows::core::Interface::as_raw(self), format, samplecount, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO) {
        (::windows::core::Interface::vtable(self).base__.CheckCounterInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcounterinfo))
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: *mut u32, szunits: ::windows::core::PSTR, punitslength: *mut u32, szdescription: ::windows::core::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CheckCounter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(ptype), ::core::mem::transmute(pactivecounters), ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength)).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetCreationFlags)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<'a, P0>(&self, hresource: P0, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.OpenSharedResource)(::windows::core::Interface::as_raw(self), hresource.into(), ::core::mem::transmute(returnedinterface), ::core::mem::transmute(ppresource)).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (::windows::core::Interface::vtable(self).base__.SetTextFilterSize)(::windows::core::Interface::as_raw(self), width, height)
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32) {
        (::windows::core::Interface::vtable(self).base__.GetTextFilterSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<'a, P0>(&self, presource: P0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D10ShaderResourceView1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Resource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateShaderResourceView1)(::windows::core::Interface::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderResourceView1>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1) -> ::windows::core::Result<ID3D10BlendState1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlendState1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10BlendState1>(result__)
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1 {
        (::windows::core::Interface::vtable(self).GetFeatureLevel)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10Device1> for ::windows::core::IUnknown {
    fn from(value: ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Device1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Device1> for ID3D10Device {
    fn from(value: ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Device1> for &'a ID3D10Device {
    fn from(value: &'a ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device1> for ID3D10Device {
    fn from(value: &ID3D10Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Device1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Device1 {}
impl ::core::fmt::Debug for ID3D10Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Device1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Device1 {}
unsafe impl ::core::marker::Sync for ID3D10Device1 {}
unsafe impl ::windows::core::Interface for ID3D10Device1 {
    type Vtable = ID3D10Device1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c8f_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device1_Vtbl {
    pub base__: ID3D10Device_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateBlendState1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateBlendState1: usize,
    pub GetFeatureLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10DeviceChild(::windows::core::IUnknown);
impl ID3D10DeviceChild {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10DeviceChild> for ::windows::core::IUnknown {
    fn from(value: ID3D10DeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10DeviceChild> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10DeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DeviceChild> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DeviceChild) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10DeviceChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10DeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DeviceChild {}
impl ::core::fmt::Debug for ID3D10DeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DeviceChild").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10DeviceChild {}
unsafe impl ::core::marker::Sync for ID3D10DeviceChild {}
unsafe impl ::windows::core::Interface for ID3D10DeviceChild {
    type Vtable = ID3D10DeviceChild_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c00_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DeviceChild_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Effect(::windows::core::IUnknown);
impl ID3D10Effect {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPool(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsPool)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Device>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).GetConstantBufferByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectConstantBuffer>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetConstantBufferByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetVariableByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetVariableByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetVariableBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetVariableBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetTechniqueByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
        (::windows::core::Interface::vtable(self).GetTechniqueByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetTechniqueByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectTechnique>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetTechniqueByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn Optimize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Optimize)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptimized(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsOptimized)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10Effect> for ::windows::core::IUnknown {
    fn from(value: ID3D10Effect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Effect> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Effect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Effect> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Effect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Effect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Effect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Effect {}
impl ::core::fmt::Debug for ID3D10Effect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Effect").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Effect {}
unsafe impl ::core::marker::Sync for ID3D10Effect {}
unsafe impl ::windows::core::Interface for ID3D10Effect {
    type Vtable = ID3D10Effect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51b0ca8b_ec0b_4519_870d_8ee1cb5017c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Effect_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPool: usize,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub GetVariableByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetVariableBySemantic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetTechniqueByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique>,
    pub GetTechniqueByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectTechnique>,
    pub Optimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptimized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptimized: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectBlendVariable(::windows::core::IUnknown);
impl ID3D10EffectBlendVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetBlendState(&self, index: u32) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBlendState)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10BlendState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBackingStore)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(pblenddesc)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectBlendVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectBlendVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectBlendVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectBlendVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectBlendVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectBlendVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectBlendVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectBlendVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectBlendVariable {}
impl ::core::fmt::Debug for ID3D10EffectBlendVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectBlendVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectBlendVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectBlendVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectBlendVariable {
    type Vtable = ID3D10EffectBlendVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fcd2294_df6d_4eae_86b3_0e9160cfb07b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectBlendVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetBlendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackingStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackingStore: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectConstantBuffer(::windows::core::IUnknown);
impl ID3D10EffectConstantBuffer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetConstantBuffer<'a, P0>(&self, pconstantbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10Buffer>>,
    {
        (::windows::core::Interface::vtable(self).SetConstantBuffer)(::windows::core::Interface::as_raw(self), pconstantbuffer.into().abi()).ok()
    }
    pub unsafe fn GetConstantBuffer(&self) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetConstantBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Buffer>(result__)
    }
    pub unsafe fn SetTextureBuffer<'a, P0>(&self, ptexturebuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10ShaderResourceView>>,
    {
        (::windows::core::Interface::vtable(self).SetTextureBuffer)(::windows::core::Interface::as_raw(self), ptexturebuffer.into().abi()).ok()
    }
    pub unsafe fn GetTextureBuffer(&self) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTextureBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderResourceView>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectConstantBuffer> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectConstantBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectConstantBuffer> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectConstantBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectConstantBuffer> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectConstantBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectConstantBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectConstantBuffer {}
impl ::core::fmt::Debug for ID3D10EffectConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectConstantBuffer").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectConstantBuffer {}
unsafe impl ::core::marker::Sync for ID3D10EffectConstantBuffer {}
unsafe impl ::windows::core::Interface for ID3D10EffectConstantBuffer {
    type Vtable = ID3D10EffectConstantBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56648f4d_cc8b_4444_a5ad_b5a3d76e91b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectConstantBuffer_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstantbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTextureBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptexturebuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTextureBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilVariable(::windows::core::IUnknown);
impl ID3D10EffectDepthStencilVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetDepthStencilState(&self, index: u32) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDepthStencilState)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_DEPTH_STENCIL_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBackingStore)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_DEPTH_STENCIL_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectDepthStencilVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectDepthStencilVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectDepthStencilVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectDepthStencilVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectDepthStencilVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectDepthStencilVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectDepthStencilVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectDepthStencilVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectDepthStencilVariable {}
impl ::core::fmt::Debug for ID3D10EffectDepthStencilVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectDepthStencilVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectDepthStencilVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectDepthStencilVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectDepthStencilVariable {
    type Vtable = ID3D10EffectDepthStencilVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf482368_330a_46a5_9a5c_01c71af24c8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetDepthStencilState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackingStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackingStore: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilViewVariable(::windows::core::IUnknown);
impl ID3D10EffectDepthStencilViewVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetDepthStencil<'a, P0>(&self, presource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10DepthStencilView>>,
    {
        (::windows::core::Interface::vtable(self).SetDepthStencil)(::windows::core::Interface::as_raw(self), presource.into().abi()).ok()
    }
    pub unsafe fn GetDepthStencil(&self) -> ::windows::core::Result<ID3D10DepthStencilView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDepthStencil)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10DepthStencilView>(result__)
    }
    pub unsafe fn SetDepthStencilArray(&self, ppresources: &[::core::option::Option<ID3D10DepthStencilView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDepthStencilArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
    pub unsafe fn GetDepthStencilArray(&self, ppresources: &mut [::core::option::Option<ID3D10DepthStencilView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDepthStencilArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
}
impl ::core::convert::From<ID3D10EffectDepthStencilViewVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectDepthStencilViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectDepthStencilViewVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectDepthStencilViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectDepthStencilViewVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectDepthStencilViewVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectDepthStencilViewVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectDepthStencilViewVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectDepthStencilViewVariable {}
impl ::core::fmt::Debug for ID3D10EffectDepthStencilViewVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectDepthStencilViewVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectDepthStencilViewVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectDepthStencilViewVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectDepthStencilViewVariable {
    type Vtable = ID3D10EffectDepthStencilViewVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e02c918_cc79_4985_b622_2d92ad701623);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetDepthStencil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDepthStencil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDepthStencilArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetDepthStencilArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectMatrixVariable(::windows::core::IUnknown);
impl ID3D10EffectMatrixVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMatrix)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMatrixArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn SetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixTranspose)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMatrixTranspose)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixTransposeArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMatrixTransposeArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
}
impl ::core::convert::From<ID3D10EffectMatrixVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectMatrixVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectMatrixVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectMatrixVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectMatrixVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectMatrixVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectMatrixVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectMatrixVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectMatrixVariable {}
impl ::core::fmt::Debug for ID3D10EffectMatrixVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectMatrixVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectMatrixVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectMatrixVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectMatrixVariable {
    type Vtable = ID3D10EffectMatrixVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50666c24_b82f_4eed_a172_5b6e7e8522e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectMatrixVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub GetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub SetMatrixArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetMatrixArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub SetMatrixTranspose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub GetMatrixTranspose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub SetMatrixTransposeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetMatrixTransposeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectPass(::windows::core::IUnknown);
impl ID3D10EffectPass {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVertexShaderDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGeometryShaderDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelShaderDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn Apply(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Apply)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ComputeStateBlockMask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
}
impl ::core::clone::Clone for ID3D10EffectPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectPass {}
impl ::core::fmt::Debug for ID3D10EffectPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectPass").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectPass {}
unsafe impl ::core::marker::Sync for ID3D10EffectPass {}
unsafe impl ::windows::core::Interface for ID3D10EffectPass {
    type Vtable = ID3D10EffectPass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cfbeb89_1a06_46e0_b282_e3f9bfa36a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPass_Vtbl {
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT,
    pub GetVertexShaderDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub GetGeometryShaderDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub GetPixelShaderDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub ComputeStateBlockMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectPool(::windows::core::IUnknown);
impl ID3D10EffectPool {
    pub unsafe fn AsEffect(&self) -> ::core::option::Option<ID3D10Effect> {
        (::windows::core::Interface::vtable(self).AsEffect)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10EffectPool> for ::windows::core::IUnknown {
    fn from(value: ID3D10EffectPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectPool> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10EffectPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectPool> for ::windows::core::IUnknown {
    fn from(value: &ID3D10EffectPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectPool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectPool {}
impl ::core::fmt::Debug for ID3D10EffectPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectPool").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectPool {}
unsafe impl ::core::marker::Sync for ID3D10EffectPool {}
unsafe impl ::windows::core::Interface for ID3D10EffectPool {
    type Vtable = ID3D10EffectPool_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9537ab04_3250_412e_8213_fcd2f8677933);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPool_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AsEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectRasterizerVariable(::windows::core::IUnknown);
impl ID3D10EffectRasterizerVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRasterizerState(&self, index: u32) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRasterizerState)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RasterizerState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_RASTERIZER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBackingStore)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_RASTERIZER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectRasterizerVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectRasterizerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectRasterizerVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectRasterizerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectRasterizerVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectRasterizerVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectRasterizerVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectRasterizerVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectRasterizerVariable {}
impl ::core::fmt::Debug for ID3D10EffectRasterizerVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectRasterizerVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectRasterizerVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectRasterizerVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectRasterizerVariable {
    type Vtable = ID3D10EffectRasterizerVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21af9f0e_4d94_4ea9_9785_2cb76b8c0b34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRasterizerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetRasterizerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackingStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackingStore: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectRenderTargetViewVariable(::windows::core::IUnknown);
impl ID3D10EffectRenderTargetViewVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetRenderTarget<'a, P0>(&self, presource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10RenderTargetView>>,
    {
        (::windows::core::Interface::vtable(self).SetRenderTarget)(::windows::core::Interface::as_raw(self), presource.into().abi()).ok()
    }
    pub unsafe fn GetRenderTarget(&self) -> ::windows::core::Result<ID3D10RenderTargetView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRenderTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10RenderTargetView>(result__)
    }
    pub unsafe fn SetRenderTargetArray(&self, ppresources: &[::core::option::Option<ID3D10RenderTargetView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRenderTargetArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
    pub unsafe fn GetRenderTargetArray(&self, ppresources: &mut [::core::option::Option<ID3D10RenderTargetView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRenderTargetArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
}
impl ::core::convert::From<ID3D10EffectRenderTargetViewVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectRenderTargetViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectRenderTargetViewVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectRenderTargetViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectRenderTargetViewVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectRenderTargetViewVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectRenderTargetViewVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectRenderTargetViewVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectRenderTargetViewVariable {}
impl ::core::fmt::Debug for ID3D10EffectRenderTargetViewVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectRenderTargetViewVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectRenderTargetViewVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectRenderTargetViewVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectRenderTargetViewVariable {
    type Vtable = ID3D10EffectRenderTargetViewVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28ca0cc3_c2c9_40bb_b57f_67b737122b17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRenderTargetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetRenderTargetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectSamplerVariable(::windows::core::IUnknown);
impl ID3D10EffectSamplerVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetSampler(&self, index: u32) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSampler)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_SAMPLER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBackingStore)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SAMPLER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectSamplerVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectSamplerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectSamplerVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectSamplerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectSamplerVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectSamplerVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectSamplerVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectSamplerVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectSamplerVariable {}
impl ::core::fmt::Debug for ID3D10EffectSamplerVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectSamplerVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectSamplerVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectSamplerVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectSamplerVariable {
    type Vtable = ID3D10EffectSamplerVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6530d5c7_07e9_4271_a418_e7ce4bd1e480);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectSamplerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetSampler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectScalarVariable(::windows::core::IUnknown);
impl ID3D10EffectScalarVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetFloat(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFloat)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetFloat(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFloat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetFloatArray(&self, pdata: &[f32], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFloatArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
    pub unsafe fn GetFloatArray(&self, pdata: &mut [f32], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFloatArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
    pub unsafe fn SetInt(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInt)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetInt(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInt)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIntArray(&self, pdata: &[i32], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
    pub unsafe fn GetIntArray(&self, pdata: &mut [i32], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIntArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBool<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBool)(::windows::core::Interface::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBool(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBool)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolArray(&self, pdata: &[super::super::Foundation::BOOL], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoolArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolArray(&self, pdata: &mut [super::super::Foundation::BOOL], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBoolArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pdata)), offset, pdata.len() as _).ok()
    }
}
impl ::core::convert::From<ID3D10EffectScalarVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectScalarVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectScalarVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectScalarVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectScalarVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectScalarVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectScalarVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectScalarVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectScalarVariable {}
impl ::core::fmt::Debug for ID3D10EffectScalarVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectScalarVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectScalarVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectScalarVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectScalarVariable {
    type Vtable = ID3D10EffectScalarVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e48f7b_d2c8_49e8_a86c_022dee53431f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectScalarVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub GetFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub SetFloatArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetFloatArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub SetInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub GetInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetIntArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetIntArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBoolArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBoolArray: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoolArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoolArray: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectShaderResourceVariable(::windows::core::IUnknown);
impl ID3D10EffectShaderResourceVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn SetResource<'a, P0>(&self, presource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ID3D10ShaderResourceView>>,
    {
        (::windows::core::Interface::vtable(self).SetResource)(::windows::core::Interface::as_raw(self), presource.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    pub unsafe fn SetResourceArray(&self, ppresources: &[::core::option::Option<ID3D10ShaderResourceView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResourceArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
    pub unsafe fn GetResourceArray(&self, ppresources: &mut [::core::option::Option<ID3D10ShaderResourceView>], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResourceArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppresources)), offset, ppresources.len() as _).ok()
    }
}
impl ::core::convert::From<ID3D10EffectShaderResourceVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectShaderResourceVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectShaderResourceVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectShaderResourceVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectShaderResourceVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectShaderResourceVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectShaderResourceVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectShaderResourceVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectShaderResourceVariable {}
impl ::core::fmt::Debug for ID3D10EffectShaderResourceVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectShaderResourceVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectShaderResourceVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectShaderResourceVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectShaderResourceVariable {
    type Vtable = ID3D10EffectShaderResourceVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0a7157b_d872_4b1d_8073_efc2acd4b1fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderResourceVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetResourceArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetResourceArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectShaderVariable(::windows::core::IUnknown);
impl ID3D10EffectShaderVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetShaderDesc(&self, shaderindex: u32) -> ::windows::core::Result<D3D10_EFFECT_SHADER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetShaderDesc)(::windows::core::Interface::as_raw(self), shaderindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_SHADER_DESC>(result__)
    }
    pub unsafe fn GetVertexShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetVertexShader)(::windows::core::Interface::as_raw(self), shaderindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn GetGeometryShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGeometryShader)(::windows::core::Interface::as_raw(self), shaderindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn GetPixelShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPixelShader)(::windows::core::Interface::as_raw(self), shaderindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10PixelShader>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputSignatureElementDesc)(::windows::core::Interface::as_raw(self), shaderindex, element, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputSignatureElementDesc)(::windows::core::Interface::as_raw(self), shaderindex, element, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectShaderVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectShaderVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectShaderVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectShaderVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectShaderVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectShaderVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectShaderVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectShaderVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectShaderVariable {}
impl ::core::fmt::Debug for ID3D10EffectShaderVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectShaderVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectShaderVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectShaderVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectShaderVariable {
    type Vtable = ID3D10EffectShaderVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80849279_c799_4797_8c33_0407a07d9e06);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetShaderDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetShaderDesc: usize,
    pub GetVertexShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGeometryShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputSignatureElementDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputSignatureElementDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputSignatureElementDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputSignatureElementDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectStringVariable(::windows::core::IUnknown);
impl ID3D10EffectStringVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetString(&self) -> ::windows::core::Result<::windows::core::PSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PSTR>(result__)
    }
    pub unsafe fn GetStringArray(&self, ppstrings: &mut [::windows::core::PSTR], offset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStringArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppstrings)), offset, ppstrings.len() as _).ok()
    }
}
impl ::core::convert::From<ID3D10EffectStringVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectStringVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectStringVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectStringVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectStringVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectStringVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectStringVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectStringVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectStringVariable {}
impl ::core::fmt::Debug for ID3D10EffectStringVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectStringVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectStringVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectStringVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectStringVariable {
    type Vtable = ID3D10EffectStringVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71417501_8df9_4e0a_a78a_255f9756baff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectStringVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstring: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrings: *mut ::windows::core::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectTechnique(::windows::core::IUnknown);
impl ID3D10EffectTechnique {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetPassByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
        (::windows::core::Interface::vtable(self).GetPassByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetPassByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectPass>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPassByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ComputeStateBlockMask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
}
impl ::core::clone::Clone for ID3D10EffectTechnique {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectTechnique {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectTechnique {}
impl ::core::fmt::Debug for ID3D10EffectTechnique {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectTechnique").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectTechnique {}
unsafe impl ::core::marker::Sync for ID3D10EffectTechnique {}
unsafe impl ::windows::core::Interface for ID3D10EffectTechnique {
    type Vtable = ID3D10EffectTechnique_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb122ce8_d1c9_4292_b237_24ed3de8b175);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectTechnique_Vtbl {
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetPassByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass>,
    pub GetPassByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectPass>,
    pub ComputeStateBlockMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectType(::windows::core::IUnknown);
impl ID3D10EffectType {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).GetMemberTypeByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectType>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMemberTypeByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberTypeBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectType>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMemberTypeBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetMemberName(&self, index: u32) -> ::windows::core::PSTR {
        (::windows::core::Interface::vtable(self).GetMemberName)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberSemantic(&self, index: u32) -> ::windows::core::PSTR {
        (::windows::core::Interface::vtable(self).GetMemberSemantic)(::windows::core::Interface::as_raw(self), index)
    }
}
impl ::core::clone::Clone for ID3D10EffectType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectType {}
impl ::core::fmt::Debug for ID3D10EffectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectType").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectType {}
unsafe impl ::core::marker::Sync for ID3D10EffectType {}
unsafe impl ::windows::core::Interface for ID3D10EffectType {
    type Vtable = ID3D10EffectType_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9e1ddc_cd9d_4772_a837_00180b9b88fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectType_Vtbl {
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType>,
    pub GetMemberTypeBySemantic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectType>,
    pub GetMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR,
    pub GetMemberSemantic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectVariable(::windows::core::IUnknown);
impl ID3D10EffectVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
}
impl ::core::clone::Clone for ID3D10EffectVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectVariable {}
impl ::core::fmt::Debug for ID3D10EffectVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectVariable {
    type Vtable = ID3D10EffectVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae897105_00e6_45bf_bb8e_281dd6db8e1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVariable_Vtbl {
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10EffectVectorVariable(::windows::core::IUnknown);
impl ID3D10EffectVectorVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).base__.IsValid)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAnnotationByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetMemberByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberBySemantic<'a, P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMemberBySemantic)(::windows::core::Interface::as_raw(self), semantic.into())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Interface::vtable(self).base__.GetElement)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Interface::vtable(self).base__.AsScalar)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Interface::vtable(self).base__.AsVector)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Interface::vtable(self).base__.AsMatrix)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Interface::vtable(self).base__.AsString)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShaderResource)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRenderTargetView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencilView)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Interface::vtable(self).base__.AsConstantBuffer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Interface::vtable(self).base__.AsShader)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Interface::vtable(self).base__.AsBlend)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Interface::vtable(self).base__.AsDepthStencil)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsRasterizer)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Interface::vtable(self).base__.AsSampler)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRawValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, bytecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoolVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFloatVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBoolVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIntVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFloatVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBoolVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFloatVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBoolVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIntVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
    pub unsafe fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFloatVectorArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), offset, count).ok()
    }
}
impl ::core::convert::From<ID3D10EffectVectorVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectVectorVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10EffectVectorVariable> for &'a ID3D10EffectVariable {
    fn from(value: &'a ID3D10EffectVectorVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectVectorVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectVectorVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10EffectVectorVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectVectorVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectVectorVariable {}
impl ::core::fmt::Debug for ID3D10EffectVectorVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectVectorVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10EffectVectorVariable {}
unsafe impl ::core::marker::Sync for ID3D10EffectVectorVariable {}
unsafe impl ::windows::core::Interface for ID3D10EffectVectorVariable {
    type Vtable = ID3D10EffectVectorVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b98c44_1f82_4c67_bcd0_72cf8f217e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVectorVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBoolVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBoolVector: usize,
    pub SetIntVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT,
    pub SetFloatVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoolVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoolVector: usize,
    pub GetIntVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT,
    pub GetFloatVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBoolVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBoolVectorArray: usize,
    pub SetIntVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub SetFloatVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoolVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoolVectorArray: usize,
    pub GetIntVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub GetFloatVectorArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10GeometryShader(::windows::core::IUnknown);
impl ID3D10GeometryShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10GeometryShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10GeometryShader> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10GeometryShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10GeometryShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10GeometryShader> for ID3D10DeviceChild {
    fn from(value: ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10GeometryShader> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10GeometryShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10GeometryShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10GeometryShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10GeometryShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10GeometryShader {}
impl ::core::fmt::Debug for ID3D10GeometryShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10GeometryShader").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10GeometryShader {}
unsafe impl ::core::marker::Sync for ID3D10GeometryShader {}
unsafe impl ::windows::core::Interface for ID3D10GeometryShader {
    type Vtable = ID3D10GeometryShader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6316be88_54cd_4040_ab44_20461bc81f68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10GeometryShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10InfoQueue(::windows::core::IUnknown);
impl ID3D10InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageCountLimit)(::windows::core::Interface::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (::windows::core::Interface::vtable(self).ClearStoredMessages)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMessage)(::windows::core::Interface::as_raw(self), messageindex, ::core::mem::transmute(pmessage), ::core::mem::transmute(pmessagebytelength)).ok()
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumStoredMessages)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        (::windows::core::Interface::vtable(self).GetMessageCountLimit)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStorageFilterEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    pub unsafe fn ClearStorageFilter(&self) {
        (::windows::core::Interface::vtable(self).ClearStorageFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyStorageFilter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfStorageFilter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushStorageFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (::windows::core::Interface::vtable(self).PopStorageFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetStorageFilterStackSize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRetrievalFilterEntries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        (::windows::core::Interface::vtable(self).ClearRetrievalFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushEmptyRetrievalFilter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushCopyOfRetrievalFilter)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushRetrievalFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (::windows::core::Interface::vtable(self).PopRetrievalFilter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetRetrievalFilterStackSize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AddMessage<'a, P0>(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMessage)(::windows::core::Interface::as_raw(self), category, severity, id, pdescription.into()).ok()
    }
    pub unsafe fn AddApplicationMessage<'a, P0>(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddApplicationMessage)(::windows::core::Interface::as_raw(self), severity, pdescription.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<'a, P0>(&self, category: D3D10_MESSAGE_CATEGORY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnCategory)(::windows::core::Interface::as_raw(self), category, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<'a, P0>(&self, severity: D3D10_MESSAGE_SEVERITY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnSeverity)(::windows::core::Interface::as_raw(self), severity, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<'a, P0>(&self, id: D3D10_MESSAGE_ID, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBreakOnID)(::windows::core::Interface::as_raw(self), id, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnCategory)(::windows::core::Interface::as_raw(self), category)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnSeverity)(::windows::core::Interface::as_raw(self), severity)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetBreakOnID)(::windows::core::Interface::as_raw(self), id)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<'a, P0>(&self, bmute: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMuteDebugOutput)(::windows::core::Interface::as_raw(self), bmute.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetMuteDebugOutput)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10InfoQueue> for ::windows::core::IUnknown {
    fn from(value: ID3D10InfoQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10InfoQueue> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10InfoQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InfoQueue> for ::windows::core::IUnknown {
    fn from(value: &ID3D10InfoQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10InfoQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10InfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10InfoQueue {}
impl ::core::fmt::Debug for ID3D10InfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10InfoQueue").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10InfoQueue {}
unsafe impl ::core::marker::Sync for ID3D10InfoQueue {}
unsafe impl ::windows::core::Interface for ID3D10InfoQueue {
    type Vtable = ID3D10InfoQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b940b17_2642_4d1f_ab1f_b99bad0c395f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InfoQueue_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMuteDebugOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10InputLayout(::windows::core::IUnknown);
impl ID3D10InputLayout {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10InputLayout> for ::windows::core::IUnknown {
    fn from(value: ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10InputLayout> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InputLayout> for ::windows::core::IUnknown {
    fn from(value: &ID3D10InputLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10InputLayout> for ID3D10DeviceChild {
    fn from(value: ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10InputLayout> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InputLayout> for ID3D10DeviceChild {
    fn from(value: &ID3D10InputLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10InputLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10InputLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10InputLayout {}
impl ::core::fmt::Debug for ID3D10InputLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10InputLayout").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10InputLayout {}
unsafe impl ::core::marker::Sync for ID3D10InputLayout {}
unsafe impl ::windows::core::Interface for ID3D10InputLayout {
    type Vtable = ID3D10InputLayout_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0b_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InputLayout_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Multithread(::windows::core::IUnknown);
impl ID3D10Multithread {
    pub unsafe fn Enter(&self) {
        (::windows::core::Interface::vtable(self).Enter)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Leave(&self) {
        (::windows::core::Interface::vtable(self).Leave)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMultithreadProtected<'a, P0>(&self, bmtprotect: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMultithreadProtected)(::windows::core::Interface::as_raw(self), bmtprotect.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetMultithreadProtected)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10Multithread> for ::windows::core::IUnknown {
    fn from(value: ID3D10Multithread) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Multithread> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Multithread) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Multithread> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Multithread) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Multithread {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Multithread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Multithread {}
impl ::core::fmt::Debug for ID3D10Multithread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Multithread").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Multithread {}
unsafe impl ::core::marker::Sync for ID3D10Multithread {}
unsafe impl ::windows::core::Interface for ID3D10Multithread {
    type Vtable = ID3D10Multithread_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Multithread_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Enter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultithreadProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultithreadProtected: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMultithreadProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMultithreadProtected: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10PixelShader(::windows::core::IUnknown);
impl ID3D10PixelShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10PixelShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10PixelShader> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10PixelShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10PixelShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10PixelShader> for ID3D10DeviceChild {
    fn from(value: ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10PixelShader> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10PixelShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10PixelShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10PixelShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10PixelShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10PixelShader {}
impl ::core::fmt::Debug for ID3D10PixelShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10PixelShader").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10PixelShader {}
unsafe impl ::core::marker::Sync for ID3D10PixelShader {}
unsafe impl ::windows::core::Interface for ID3D10PixelShader {
    type Vtable = ID3D10PixelShader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4968b601_9d00_4cde_8346_8e7f675819b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10PixelShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Predicate(::windows::core::IUnknown);
impl ID3D10Predicate {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.Begin)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).base__.base__.End)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.base__.GetDataSize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC) {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Predicate> for ::windows::core::IUnknown {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Predicate> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10DeviceChild {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Predicate> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10DeviceChild {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10Asynchronous {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Predicate> for &'a ID3D10Asynchronous {
    fn from(value: &'a ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10Asynchronous {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10Query {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Predicate> for &'a ID3D10Query {
    fn from(value: &'a ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10Query {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Predicate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Predicate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Predicate {}
impl ::core::fmt::Debug for ID3D10Predicate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Predicate").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Predicate {}
unsafe impl ::core::marker::Sync for ID3D10Predicate {}
unsafe impl ::windows::core::Interface for ID3D10Predicate {
    type Vtable = ID3D10Predicate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c10_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Predicate_Vtbl {
    pub base__: ID3D10Query_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Query(::windows::core::IUnknown);
impl ID3D10Query {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).base__.Begin)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).base__.End)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetDataSize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Query> for ::windows::core::IUnknown {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Query> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Query> for ID3D10DeviceChild {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Query> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ID3D10DeviceChild {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Query> for ID3D10Asynchronous {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Query> for &'a ID3D10Asynchronous {
    fn from(value: &'a ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ID3D10Asynchronous {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Query {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Query {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Query {}
impl ::core::fmt::Debug for ID3D10Query {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Query").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Query {}
unsafe impl ::core::marker::Sync for ID3D10Query {}
unsafe impl ::windows::core::Interface for ID3D10Query {
    type Vtable = ID3D10Query_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0e_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Query_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10RasterizerState(::windows::core::IUnknown);
impl ID3D10RasterizerState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10RasterizerState> for ::windows::core::IUnknown {
    fn from(value: ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10RasterizerState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RasterizerState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10RasterizerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10RasterizerState> for ID3D10DeviceChild {
    fn from(value: ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10RasterizerState> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RasterizerState> for ID3D10DeviceChild {
    fn from(value: &ID3D10RasterizerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10RasterizerState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10RasterizerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10RasterizerState {}
impl ::core::fmt::Debug for ID3D10RasterizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10RasterizerState").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10RasterizerState {}
unsafe impl ::core::marker::Sync for ID3D10RasterizerState {}
unsafe impl ::windows::core::Interface for ID3D10RasterizerState {
    type Vtable = ID3D10RasterizerState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2a07292_89af_4345_be2e_c53d9fbb6e9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RasterizerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10RenderTargetView(::windows::core::IUnknown);
impl ID3D10RenderTargetView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).base__.GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresource))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ::windows::core::IUnknown {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10RenderTargetView> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ID3D10DeviceChild {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10RenderTargetView> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ID3D10DeviceChild {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ID3D10View {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10RenderTargetView> for &'a ID3D10View {
    fn from(value: &'a ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ID3D10View {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10RenderTargetView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10RenderTargetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10RenderTargetView {}
impl ::core::fmt::Debug for ID3D10RenderTargetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10RenderTargetView").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10RenderTargetView {}
unsafe impl ::core::marker::Sync for ID3D10RenderTargetView {}
unsafe impl ::windows::core::Interface for ID3D10RenderTargetView {
    type Vtable = ID3D10RenderTargetView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c08_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RenderTargetView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Resource(::windows::core::IUnknown);
impl ID3D10Resource {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetEvictionPriority)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10Resource> for ::windows::core::IUnknown {
    fn from(value: ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Resource> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Resource> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Resource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Resource> for ID3D10DeviceChild {
    fn from(value: ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Resource> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Resource> for ID3D10DeviceChild {
    fn from(value: &ID3D10Resource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Resource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Resource {}
impl ::core::fmt::Debug for ID3D10Resource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Resource").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Resource {}
unsafe impl ::core::marker::Sync for ID3D10Resource {}
unsafe impl ::windows::core::Interface for ID3D10Resource {
    type Vtable = ID3D10Resource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c01_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Resource_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10SamplerState(::windows::core::IUnknown);
impl ID3D10SamplerState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10SamplerState> for ::windows::core::IUnknown {
    fn from(value: ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10SamplerState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SamplerState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10SamplerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10SamplerState> for ID3D10DeviceChild {
    fn from(value: ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10SamplerState> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SamplerState> for ID3D10DeviceChild {
    fn from(value: &ID3D10SamplerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10SamplerState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10SamplerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10SamplerState {}
impl ::core::fmt::Debug for ID3D10SamplerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10SamplerState").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10SamplerState {}
unsafe impl ::core::marker::Sync for ID3D10SamplerState {}
unsafe impl ::windows::core::Interface for ID3D10SamplerState {
    type Vtable = ID3D10SamplerState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0c_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SamplerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderReflection(::windows::core::IUnknown);
impl ID3D10ShaderReflection {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        (::windows::core::Interface::vtable(self).GetConstantBufferByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetConstantBufferByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceBindingDesc)(::windows::core::Interface::as_raw(self), resourceindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputParameterDesc)(::windows::core::Interface::as_raw(self), parameterindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputParameterDesc)(::windows::core::Interface::as_raw(self), parameterindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10ShaderReflection> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderReflection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderReflection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10ShaderReflection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderReflection> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderReflection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10ShaderReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflection {}
impl ::core::fmt::Debug for ID3D10ShaderReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflection").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderReflection {}
unsafe impl ::core::marker::Sync for ID3D10ShaderReflection {}
unsafe impl ::windows::core::Interface for ID3D10ShaderReflection {
    type Vtable = ID3D10ShaderReflection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd40e20b6_f8f7_42ad_ab20_4baf8f15dfaa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderReflection1(::windows::core::IUnknown);
impl ID3D10ShaderReflection1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        (::windows::core::Interface::vtable(self).GetConstantBufferByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetConstantBufferByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceBindingDesc)(::windows::core::Interface::as_raw(self), resourceindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputParameterDesc)(::windows::core::Interface::as_raw(self), parameterindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputParameterDesc)(::windows::core::Interface::as_raw(self), parameterindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    pub unsafe fn GetVariableByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10ShaderReflectionVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetVariableByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<'a, P0>(&self, name: P0) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetResourceBindingDescByName)(::windows::core::Interface::as_raw(self), name.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    pub unsafe fn GetMovInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMovInstructionCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMovcInstructionCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetConversionInstructionCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBitwiseInstructionCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> ::windows::core::Result<super::Direct3D::D3D_PRIMITIVE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGSInputPrimitive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Direct3D::D3D_PRIMITIVE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLevel9Shader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLevel9Shader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSampleFrequencyShader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSampleFrequencyShader)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ID3D10ShaderReflection1> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderReflection1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderReflection1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10ShaderReflection1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderReflection1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderReflection1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10ShaderReflection1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflection1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflection1 {}
impl ::core::fmt::Debug for ID3D10ShaderReflection1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflection1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderReflection1 {}
unsafe impl ::core::marker::Sync for ID3D10ShaderReflection1 {}
unsafe impl ::windows::core::Interface for ID3D10ShaderReflection1 {
    type Vtable = ID3D10ShaderReflection1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3457783_a846_47ce_9520_cea6f66e7447);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection1_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetMovcInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetConversionInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetGSInputPrimitive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetGSInputPrimitive: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLevel9Shader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLevel9Shader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSampleFrequencyShader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSampleFrequencyShader: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderReflectionConstantBuffer(::windows::core::IUnknown);
impl ID3D10ShaderReflectionConstantBuffer {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_BUFFER_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_BUFFER_DESC>(result__)
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
        (::windows::core::Interface::vtable(self).GetVariableByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10ShaderReflectionVariable>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetVariableByName)(::windows::core::Interface::as_raw(self), name.into())
    }
}
impl ::core::clone::Clone for ID3D10ShaderReflectionConstantBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionConstantBuffer {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionConstantBuffer").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderReflectionConstantBuffer {}
unsafe impl ::core::marker::Sync for ID3D10ShaderReflectionConstantBuffer {}
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionConstantBuffer {
    type Vtable = ID3D10ShaderReflectionConstantBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66c66a94_dddd_4b62_a66a_f0da33c2b4d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderReflectionType(::windows::core::IUnknown);
impl ID3D10ShaderReflectionType {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
        (::windows::core::Interface::vtable(self).GetMemberTypeByIndex)(::windows::core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<'a, P0>(&self, name: P0) -> ::core::option::Option<ID3D10ShaderReflectionType>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMemberTypeByName)(::windows::core::Interface::as_raw(self), name.into())
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> ::windows::core::PSTR {
        (::windows::core::Interface::vtable(self).GetMemberTypeName)(::windows::core::Interface::as_raw(self), index)
    }
}
impl ::core::clone::Clone for ID3D10ShaderReflectionType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionType {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionType").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderReflectionType {}
unsafe impl ::core::marker::Sync for ID3D10ShaderReflectionType {}
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionType {
    type Vtable = ID3D10ShaderReflectionType_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc530ad7d_9b16_4395_a979_ba2ecff83add);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::PSTR,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderReflectionVariable(::windows::core::IUnknown);
impl ID3D10ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_VARIABLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_SHADER_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10ShaderReflectionType> {
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::clone::Clone for ID3D10ShaderReflectionVariable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionVariable {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionVariable").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderReflectionVariable {}
unsafe impl ::core::marker::Sync for ID3D10ShaderReflectionVariable {}
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionVariable {
    type Vtable = ID3D10ShaderReflectionVariable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf63c95_2650_405d_99c1_3636bd1da0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType>,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderResourceView(::windows::core::IUnknown);
impl ID3D10ShaderResourceView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).base__.GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresource))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ID3D10DeviceChild {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ID3D10DeviceChild {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ID3D10View {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView> for &'a ID3D10View {
    fn from(value: &'a ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ID3D10View {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10ShaderResourceView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderResourceView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderResourceView {}
impl ::core::fmt::Debug for ID3D10ShaderResourceView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderResourceView").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderResourceView {}
unsafe impl ::core::marker::Sync for ID3D10ShaderResourceView {}
unsafe impl ::windows::core::Interface for ID3D10ShaderResourceView {
    type Vtable = ID3D10ShaderResourceView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c07_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10ShaderResourceView1(::windows::core::IUnknown);
impl ID3D10ShaderResourceView1 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresource))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).base__.GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
        (::windows::core::Interface::vtable(self).GetDesc1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10DeviceChild {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView1> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10DeviceChild {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10View {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView1> for &'a ID3D10View {
    fn from(value: &'a ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10View {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10ShaderResourceView {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10ShaderResourceView1> for &'a ID3D10ShaderResourceView {
    fn from(value: &'a ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10ShaderResourceView {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10ShaderResourceView1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderResourceView1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderResourceView1 {}
impl ::core::fmt::Debug for ID3D10ShaderResourceView1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderResourceView1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10ShaderResourceView1 {}
unsafe impl ::core::marker::Sync for ID3D10ShaderResourceView1 {}
unsafe impl ::windows::core::Interface for ID3D10ShaderResourceView1 {
    type Vtable = ID3D10ShaderResourceView1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c87_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView1_Vtbl {
    pub base__: ID3D10ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10StateBlock(::windows::core::IUnknown);
impl ID3D10StateBlock {
    pub unsafe fn Capture(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Capture)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Apply)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseAllDeviceObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseAllDeviceObjects)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ID3D10Device>(result__)
    }
}
impl ::core::convert::From<ID3D10StateBlock> for ::windows::core::IUnknown {
    fn from(value: ID3D10StateBlock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10StateBlock> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10StateBlock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10StateBlock> for ::windows::core::IUnknown {
    fn from(value: &ID3D10StateBlock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10StateBlock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10StateBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10StateBlock {}
impl ::core::fmt::Debug for ID3D10StateBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10StateBlock").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10StateBlock {}
unsafe impl ::core::marker::Sync for ID3D10StateBlock {}
unsafe impl ::windows::core::Interface for ID3D10StateBlock {
    type Vtable = ID3D10StateBlock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0803425a_57f5_4dd6_9465_a87570834a08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10StateBlock_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Capture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseAllDeviceObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10SwitchToRef(::windows::core::IUnknown);
impl ID3D10SwitchToRef {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseRef<'a, P0>(&self, useref: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetUseRef)(::windows::core::Interface::as_raw(self), useref.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseRef(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).GetUseRef)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ID3D10SwitchToRef> for ::windows::core::IUnknown {
    fn from(value: ID3D10SwitchToRef) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10SwitchToRef> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10SwitchToRef) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SwitchToRef> for ::windows::core::IUnknown {
    fn from(value: &ID3D10SwitchToRef) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10SwitchToRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10SwitchToRef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10SwitchToRef {}
impl ::core::fmt::Debug for ID3D10SwitchToRef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10SwitchToRef").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10SwitchToRef {}
unsafe impl ::core::marker::Sync for ID3D10SwitchToRef {}
unsafe impl ::windows::core::Interface for ID3D10SwitchToRef {
    type Vtable = ID3D10SwitchToRef_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e02_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SwitchToRef_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseRef: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseRef: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Texture1D(::windows::core::IUnknown);
impl ID3D10Texture1D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).base__.SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetEvictionPriority)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), subresource, maptype, mapflags, ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self), subresource)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture1D> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture1D> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ID3D10Resource {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture1D> for &'a ID3D10Resource {
    fn from(value: &'a ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ID3D10Resource {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Texture1D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture1D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture1D {}
impl ::core::fmt::Debug for ID3D10Texture1D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture1D").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Texture1D {}
unsafe impl ::core::marker::Sync for ID3D10Texture1D {}
unsafe impl ::windows::core::Interface for ID3D10Texture1D {
    type Vtable = ID3D10Texture1D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c03_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture1D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Texture2D(::windows::core::IUnknown);
impl ID3D10Texture2D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).base__.SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetEvictionPriority)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE2D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), subresource, maptype, mapflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_MAPPED_TEXTURE2D>(result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self), subresource)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture2D> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture2D> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ID3D10Resource {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture2D> for &'a ID3D10Resource {
    fn from(value: &'a ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ID3D10Resource {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Texture2D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture2D {}
impl ::core::fmt::Debug for ID3D10Texture2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture2D").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Texture2D {}
unsafe impl ::core::marker::Sync for ID3D10Texture2D {}
unsafe impl ::windows::core::Interface for ID3D10Texture2D {
    type Vtable = ID3D10Texture2D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c04_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture2D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10Texture3D(::windows::core::IUnknown);
impl ID3D10Texture3D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).base__.SetEvictionPriority)(::windows::core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Interface::vtable(self).base__.GetEvictionPriority)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Map)(::windows::core::Interface::as_raw(self), subresource, maptype, mapflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<D3D10_MAPPED_TEXTURE3D>(result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self), subresource)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC) {
        (::windows::core::Interface::vtable(self).GetDesc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture3D> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture3D> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ID3D10Resource {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10Texture3D> for &'a ID3D10Resource {
    fn from(value: &'a ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ID3D10Resource {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10Texture3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture3D {}
impl ::core::fmt::Debug for ID3D10Texture3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture3D").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10Texture3D {}
unsafe impl ::core::marker::Sync for ID3D10Texture3D {}
unsafe impl ::windows::core::Interface for ID3D10Texture3D {
    type Vtable = ID3D10Texture3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c05_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture3D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10VertexShader(::windows::core::IUnknown);
impl ID3D10VertexShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10VertexShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10VertexShader> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10VertexShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10VertexShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10VertexShader> for ID3D10DeviceChild {
    fn from(value: ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10VertexShader> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10VertexShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10VertexShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10VertexShader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10VertexShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10VertexShader {}
impl ::core::fmt::Debug for ID3D10VertexShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10VertexShader").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10VertexShader {}
unsafe impl ::core::marker::Sync for ID3D10VertexShader {}
unsafe impl ::windows::core::Interface for ID3D10VertexShader {
    type Vtable = ID3D10VertexShader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0a_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10VertexShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
#[repr(transparent)]
pub struct ID3D10View(::windows::core::IUnknown);
impl ID3D10View {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), datasize, ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guid), pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).GetResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppresource))
    }
}
impl ::core::convert::From<ID3D10View> for ::windows::core::IUnknown {
    fn from(value: ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10View> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10View> for ::windows::core::IUnknown {
    fn from(value: &ID3D10View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D10View> for ID3D10DeviceChild {
    fn from(value: ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D10View> for &'a ID3D10DeviceChild {
    fn from(value: &'a ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10View> for ID3D10DeviceChild {
    fn from(value: &ID3D10View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D10View {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D10View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10View {}
impl ::core::fmt::Debug for ID3D10View {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10View").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D10View {}
unsafe impl ::core::marker::Sync for ID3D10View {}
unsafe impl ::windows::core::Interface for ID3D10View {
    type Vtable = ID3D10View_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902b03f_60a7_49ba_9936_2a3ab37a7e33);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10View_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub type PFN_D3D10_CREATE_DEVICE1 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::Dxgi::DXGI_SWAP_CHAIN_DESC, param7: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, param8: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D10\"`*"]
pub const _FACD3D10: u32 = 2169u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
