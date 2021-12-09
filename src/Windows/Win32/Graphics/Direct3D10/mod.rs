#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10CompileEffectFromMemory<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::Direct3D::ID3DInclude>>(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: Param2, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: Param4, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrors: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CompileEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::core::RawPtr, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut ::windows::core::RawPtr, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        D3D10CompileEffectFromMemory(::core::mem::transmute(pdata), ::core::mem::transmute(datalength), psrcfilename.into_param().abi(), ::core::mem::transmute(pdefines), pinclude.into_param().abi(), ::core::mem::transmute(hlslflags), ::core::mem::transmute(fxflags), ::core::mem::transmute(ppcompiledeffect), ::core::mem::transmute(pperrors)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10CompileShader<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::Direct3D::ID3DInclude>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(psrcdata: Param0, srcdatasize: usize, pfilename: Param2, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: Param4, pfunctionname: Param5, pprofile: Param6, flags: u32, ppshader: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CompileShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::core::RawPtr, pfunctionname: super::super::Foundation::PSTR, pprofile: super::super::Foundation::PSTR, flags: u32, ppshader: *mut ::windows::core::RawPtr, pperrormsgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        D3D10CompileShader(psrcdata.into_param().abi(), ::core::mem::transmute(srcdatasize), pfilename.into_param().abi(), ::core::mem::transmute(pdefines), pinclude.into_param().abi(), pfunctionname.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(ppshader), ::core::mem::transmute(pperrormsgs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CreateBlob(numbytes: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateBlob(numbytes: usize, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateBlob(::core::mem::transmute(numbytes), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D10CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGIAdapter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(padapter: Param0, drivertype: D3D10_DRIVER_TYPE, software: Param2, flags: u32, sdkversion: u32) -> ::windows::core::Result<ID3D10Device> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateDevice(padapter: ::windows::core::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateDevice(padapter.into_param().abi(), ::core::mem::transmute(drivertype), software.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(sdkversion), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Device>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D10CreateDevice1<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGIAdapter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(padapter: Param0, drivertype: D3D10_DRIVER_TYPE, software: Param2, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32) -> ::windows::core::Result<ID3D10Device1> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateDevice1(padapter: ::windows::core::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateDevice1(padapter.into_param().abi(), ::core::mem::transmute(drivertype), software.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(hardwarelevel), ::core::mem::transmute(sdkversion), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Device1>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGIAdapter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(padapter: Param0, drivertype: D3D10_DRIVER_TYPE, software: Param2, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, ppdevice: *mut ::core::option::Option<ID3D10Device>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateDeviceAndSwapChain(padapter: ::windows::core::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        D3D10CreateDeviceAndSwapChain(padapter.into_param().abi(), ::core::mem::transmute(drivertype), software.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(sdkversion), ::core::mem::transmute(pswapchaindesc), ::core::mem::transmute(ppswapchain), ::core::mem::transmute(ppdevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain1<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGIAdapter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(padapter: Param0, drivertype: D3D10_DRIVER_TYPE, software: Param2, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, ppdevice: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateDeviceAndSwapChain1(padapter: ::windows::core::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        D3D10CreateDeviceAndSwapChain1(padapter.into_param().abi(), ::core::mem::transmute(drivertype), software.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(hardwarelevel), ::core::mem::transmute(sdkversion), ::core::mem::transmute(pswapchaindesc), ::core::mem::transmute(ppswapchain), ::core::mem::transmute(ppdevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10CreateEffectFromMemory<'a, Param3: ::windows::core::IntoParam<'a, ID3D10Device>, Param4: ::windows::core::IntoParam<'a, ID3D10EffectPool>>(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: Param3, peffectpool: Param4) -> ::windows::core::Result<ID3D10Effect> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ::windows::core::RawPtr, peffectpool: ::windows::core::RawPtr, ppeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateEffectFromMemory(::core::mem::transmute(pdata), ::core::mem::transmute(datalength), ::core::mem::transmute(fxflags), pdevice.into_param().abi(), peffectpool.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Effect>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10CreateEffectPoolFromMemory<'a, Param3: ::windows::core::IntoParam<'a, ID3D10Device>>(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: Param3) -> ::windows::core::Result<ID3D10EffectPool> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateEffectPoolFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ::windows::core::RawPtr, ppeffectpool: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateEffectPoolFromMemory(::core::mem::transmute(pdata), ::core::mem::transmute(datalength), ::core::mem::transmute(fxflags), pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10EffectPool>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10CreateStateBlock<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Device>>(pdevice: Param0, pstateblockmask: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<ID3D10StateBlock> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10CreateStateBlock(pdevice: ::windows::core::RawPtr, pstateblockmask: *const D3D10_STATE_BLOCK_MASK, ppstateblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10CreateStateBlock(pdevice.into_param().abi(), ::core::mem::transmute(pstateblockmask), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10StateBlock>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10DisassembleEffect<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Effect>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(peffect: Param0, enablecolorcode: Param1) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10DisassembleEffect(peffect: ::windows::core::RawPtr, enablecolorcode: super::super::Foundation::BOOL, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10DisassembleEffect(peffect.into_param().abi(), enablecolorcode.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10DisassembleShader<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: Param2, pcomments: Param3) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10DisassembleShader(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: super::super::Foundation::BOOL, pcomments: super::super::Foundation::PSTR, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10DisassembleShader(::core::mem::transmute(pshader), ::core::mem::transmute(bytecodelength), enablecolorcode.into_param().abi(), pcomments.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3D10GetGeometryShaderProfile<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Device>>(pdevice: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetGeometryShaderProfile(pdevice: ::windows::core::RawPtr) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(D3D10GetGeometryShaderProfile(pdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10GetInputAndOutputSignatureBlob(::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10GetInputSignatureBlob(::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10GetOutputSignatureBlob(::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3D10GetPixelShaderProfile<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Device>>(pdevice: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetPixelShaderProfile(pdevice: ::windows::core::RawPtr) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(D3D10GetPixelShaderProfile(pdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<super::Direct3D::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppdebuginfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10GetShaderDebugInfo(::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3D10GetVertexShaderProfile<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Device>>(pdevice: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10GetVertexShaderProfile(pdevice: ::windows::core::RawPtr) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(D3D10GetVertexShaderProfile(pdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[inline]
pub unsafe fn D3D10PreprocessShader<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::Direct3D::ID3DInclude>>(psrcdata: Param0, srcdatasize: usize, pfilename: Param2, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: Param4, ppshadertext: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10PreprocessShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::core::RawPtr, ppshadertext: *mut ::windows::core::RawPtr, pperrormsgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        D3D10PreprocessShader(psrcdata.into_param().abi(), ::core::mem::transmute(srcdatasize), pfilename.into_param().abi(), ::core::mem::transmute(pdefines), pinclude.into_param().abi(), ::core::mem::transmute(ppshadertext), ::core::mem::transmute(pperrormsgs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10ShaderReflection> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppreflector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        D3D10ReflectShader(::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderReflection>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
        }
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        D3D10StateBlockMaskDifference(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableAll() -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
        }
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        D3D10StateBlockMaskDisableAll(::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::HRESULT;
        }
        D3D10StateBlockMaskDisableCapture(::core::mem::transmute(pmask), ::core::mem::transmute(statetype), ::core::mem::transmute(rangestart), ::core::mem::transmute(rangelength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableAll() -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
        }
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        D3D10StateBlockMaskEnableAll(::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::core::HRESULT;
        }
        D3D10StateBlockMaskEnableCapture(::core::mem::transmute(pmask), ::core::mem::transmute(statetype), ::core::mem::transmute(rangestart), ::core::mem::transmute(rangelength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(D3D10StateBlockMaskGetSetting(::core::mem::transmute(pmask), ::core::mem::transmute(statetype), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
        }
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        D3D10StateBlockMaskIntersect(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT;
        }
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        D3D10StateBlockMaskUnion(::core::mem::transmute(pa), ::core::mem::transmute(pb), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1u32;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152u32;
pub const D3D10_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D10_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub type D3D10_ASYNC_GETDATA_FLAG = i32;
pub const D3D10_ASYNC_GETDATA_DONOTFLUSH: D3D10_ASYNC_GETDATA_FLAG = 1i32;
pub type D3D10_BIND_FLAG = i32;
pub const D3D10_BIND_VERTEX_BUFFER: D3D10_BIND_FLAG = 1i32;
pub const D3D10_BIND_INDEX_BUFFER: D3D10_BIND_FLAG = 2i32;
pub const D3D10_BIND_CONSTANT_BUFFER: D3D10_BIND_FLAG = 4i32;
pub const D3D10_BIND_SHADER_RESOURCE: D3D10_BIND_FLAG = 8i32;
pub const D3D10_BIND_STREAM_OUTPUT: D3D10_BIND_FLAG = 16i32;
pub const D3D10_BIND_RENDER_TARGET: D3D10_BIND_FLAG = 32i32;
pub const D3D10_BIND_DEPTH_STENCIL: D3D10_BIND_FLAG = 64i32;
pub type D3D10_BLEND = i32;
pub const D3D10_BLEND_ZERO: D3D10_BLEND = 1i32;
pub const D3D10_BLEND_ONE: D3D10_BLEND = 2i32;
pub const D3D10_BLEND_SRC_COLOR: D3D10_BLEND = 3i32;
pub const D3D10_BLEND_INV_SRC_COLOR: D3D10_BLEND = 4i32;
pub const D3D10_BLEND_SRC_ALPHA: D3D10_BLEND = 5i32;
pub const D3D10_BLEND_INV_SRC_ALPHA: D3D10_BLEND = 6i32;
pub const D3D10_BLEND_DEST_ALPHA: D3D10_BLEND = 7i32;
pub const D3D10_BLEND_INV_DEST_ALPHA: D3D10_BLEND = 8i32;
pub const D3D10_BLEND_DEST_COLOR: D3D10_BLEND = 9i32;
pub const D3D10_BLEND_INV_DEST_COLOR: D3D10_BLEND = 10i32;
pub const D3D10_BLEND_SRC_ALPHA_SAT: D3D10_BLEND = 11i32;
pub const D3D10_BLEND_BLEND_FACTOR: D3D10_BLEND = 14i32;
pub const D3D10_BLEND_INV_BLEND_FACTOR: D3D10_BLEND = 15i32;
pub const D3D10_BLEND_SRC1_COLOR: D3D10_BLEND = 16i32;
pub const D3D10_BLEND_INV_SRC1_COLOR: D3D10_BLEND = 17i32;
pub const D3D10_BLEND_SRC1_ALPHA: D3D10_BLEND = 18i32;
pub const D3D10_BLEND_INV_SRC1_ALPHA: D3D10_BLEND = 19i32;
#[repr(C)]
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
pub type D3D10_BLEND_OP = i32;
pub const D3D10_BLEND_OP_ADD: D3D10_BLEND_OP = 1i32;
pub const D3D10_BLEND_OP_SUBTRACT: D3D10_BLEND_OP = 2i32;
pub const D3D10_BLEND_OP_REV_SUBTRACT: D3D10_BLEND_OP = 3i32;
pub const D3D10_BLEND_OP_MIN: D3D10_BLEND_OP = 4i32;
pub const D3D10_BLEND_OP_MAX: D3D10_BLEND_OP = 5i32;
#[repr(C)]
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
#[repr(C)]
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
pub type D3D10_CLEAR_FLAG = i32;
pub const D3D10_CLEAR_DEPTH: D3D10_CLEAR_FLAG = 1i32;
pub const D3D10_CLEAR_STENCIL: D3D10_CLEAR_FLAG = 2i32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub type D3D10_COLOR_WRITE_ENABLE = i32;
pub const D3D10_COLOR_WRITE_ENABLE_RED: D3D10_COLOR_WRITE_ENABLE = 1i32;
pub const D3D10_COLOR_WRITE_ENABLE_GREEN: D3D10_COLOR_WRITE_ENABLE = 2i32;
pub const D3D10_COLOR_WRITE_ENABLE_BLUE: D3D10_COLOR_WRITE_ENABLE = 4i32;
pub const D3D10_COLOR_WRITE_ENABLE_ALPHA: D3D10_COLOR_WRITE_ENABLE = 8i32;
pub const D3D10_COLOR_WRITE_ENABLE_ALL: D3D10_COLOR_WRITE_ENABLE = 15i32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D10_COMPARISON_FILTERING_BIT: u32 = 128u32;
pub type D3D10_COMPARISON_FUNC = i32;
pub const D3D10_COMPARISON_NEVER: D3D10_COMPARISON_FUNC = 1i32;
pub const D3D10_COMPARISON_LESS: D3D10_COMPARISON_FUNC = 2i32;
pub const D3D10_COMPARISON_EQUAL: D3D10_COMPARISON_FUNC = 3i32;
pub const D3D10_COMPARISON_LESS_EQUAL: D3D10_COMPARISON_FUNC = 4i32;
pub const D3D10_COMPARISON_GREATER: D3D10_COMPARISON_FUNC = 5i32;
pub const D3D10_COMPARISON_NOT_EQUAL: D3D10_COMPARISON_FUNC = 6i32;
pub const D3D10_COMPARISON_GREATER_EQUAL: D3D10_COMPARISON_FUNC = 7i32;
pub const D3D10_COMPARISON_ALWAYS: D3D10_COMPARISON_FUNC = 8i32;
pub type D3D10_COUNTER = i32;
pub const D3D10_COUNTER_GPU_IDLE: D3D10_COUNTER = 0i32;
pub const D3D10_COUNTER_VERTEX_PROCESSING: D3D10_COUNTER = 1i32;
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: D3D10_COUNTER = 2i32;
pub const D3D10_COUNTER_PIXEL_PROCESSING: D3D10_COUNTER = 3i32;
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: D3D10_COUNTER = 4i32;
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: D3D10_COUNTER = 5i32;
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: D3D10_COUNTER = 6i32;
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 7i32;
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 8i32;
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 9i32;
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: D3D10_COUNTER = 10i32;
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: D3D10_COUNTER = 11i32;
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: D3D10_COUNTER = 12i32;
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: D3D10_COUNTER = 13i32;
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: D3D10_COUNTER = 14i32;
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: D3D10_COUNTER = 15i32;
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: D3D10_COUNTER = 16i32;
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: D3D10_COUNTER = 17i32;
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: D3D10_COUNTER = 1073741824i32;
#[repr(C)]
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
pub type D3D10_COUNTER_TYPE = i32;
pub const D3D10_COUNTER_TYPE_FLOAT32: D3D10_COUNTER_TYPE = 0i32;
pub const D3D10_COUNTER_TYPE_UINT16: D3D10_COUNTER_TYPE = 1i32;
pub const D3D10_COUNTER_TYPE_UINT32: D3D10_COUNTER_TYPE = 2i32;
pub const D3D10_COUNTER_TYPE_UINT64: D3D10_COUNTER_TYPE = 3i32;
pub type D3D10_CPU_ACCESS_FLAG = i32;
pub const D3D10_CPU_ACCESS_WRITE: D3D10_CPU_ACCESS_FLAG = 65536i32;
pub const D3D10_CPU_ACCESS_READ: D3D10_CPU_ACCESS_FLAG = 131072i32;
pub type D3D10_CREATE_DEVICE_FLAG = i32;
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: D3D10_CREATE_DEVICE_FLAG = 1i32;
pub const D3D10_CREATE_DEVICE_DEBUG: D3D10_CREATE_DEVICE_FLAG = 2i32;
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: D3D10_CREATE_DEVICE_FLAG = 4i32;
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D10_CREATE_DEVICE_FLAG = 8i32;
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: D3D10_CREATE_DEVICE_FLAG = 16i32;
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: D3D10_CREATE_DEVICE_FLAG = 32i32;
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D10_CREATE_DEVICE_FLAG = 128i32;
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: D3D10_CREATE_DEVICE_FLAG = 512i32;
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: D3D10_CREATE_DEVICE_FLAG = 1024i32;
pub type D3D10_CULL_MODE = i32;
pub const D3D10_CULL_NONE: D3D10_CULL_MODE = 1i32;
pub const D3D10_CULL_FRONT: D3D10_CULL_MODE = 2i32;
pub const D3D10_CULL_BACK: D3D10_CULL_MODE = 3i32;
pub const D3D10_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
pub const D3D10_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
pub const D3D10_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D10_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D10_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0u32;
pub const D3D10_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D10_DEFAULT_MAX_ANISOTROPY: f32 = 16f32;
pub const D3D10_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
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
pub type D3D10_DEPTH_WRITE_MASK = i32;
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = 0i32;
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = 1i32;
pub type D3D10_DEVICE_STATE_TYPES = i32;
pub const D3D10_DST_SO_BUFFERS: D3D10_DEVICE_STATE_TYPES = 1i32;
pub const D3D10_DST_OM_RENDER_TARGETS: D3D10_DEVICE_STATE_TYPES = 2i32;
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: D3D10_DEVICE_STATE_TYPES = 3i32;
pub const D3D10_DST_OM_BLEND_STATE: D3D10_DEVICE_STATE_TYPES = 4i32;
pub const D3D10_DST_VS: D3D10_DEVICE_STATE_TYPES = 5i32;
pub const D3D10_DST_VS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 6i32;
pub const D3D10_DST_VS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 7i32;
pub const D3D10_DST_VS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 8i32;
pub const D3D10_DST_GS: D3D10_DEVICE_STATE_TYPES = 9i32;
pub const D3D10_DST_GS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 10i32;
pub const D3D10_DST_GS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 11i32;
pub const D3D10_DST_GS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 12i32;
pub const D3D10_DST_PS: D3D10_DEVICE_STATE_TYPES = 13i32;
pub const D3D10_DST_PS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 14i32;
pub const D3D10_DST_PS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 15i32;
pub const D3D10_DST_PS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 16i32;
pub const D3D10_DST_IA_VERTEX_BUFFERS: D3D10_DEVICE_STATE_TYPES = 17i32;
pub const D3D10_DST_IA_INDEX_BUFFER: D3D10_DEVICE_STATE_TYPES = 18i32;
pub const D3D10_DST_IA_INPUT_LAYOUT: D3D10_DEVICE_STATE_TYPES = 19i32;
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: D3D10_DEVICE_STATE_TYPES = 20i32;
pub const D3D10_DST_RS_VIEWPORTS: D3D10_DEVICE_STATE_TYPES = 21i32;
pub const D3D10_DST_RS_SCISSOR_RECTS: D3D10_DEVICE_STATE_TYPES = 22i32;
pub const D3D10_DST_RS_RASTERIZER_STATE: D3D10_DEVICE_STATE_TYPES = 23i32;
pub const D3D10_DST_PREDICATION: D3D10_DEVICE_STATE_TYPES = 24i32;
pub type D3D10_DRIVER_TYPE = i32;
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = 0i32;
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = 1i32;
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = 2i32;
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = 3i32;
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = 5i32;
pub type D3D10_DSV_DIMENSION = i32;
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = 0i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = 1i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = 2i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = 3i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = 4i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = 5i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = 6i32;
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *mut u8,
    pub IsInline: super::super::Foundation::BOOL,
    pub pBytecode: *mut u8,
    pub BytecodeLength: u32,
    pub SODecl: super::super::Foundation::PSTR,
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
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: super::super::Foundation::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_EFFECT_TYPE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_EFFECT_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D10_EFFECT_TYPE_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D10_EFFECT_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_TYPE_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D10_EFFECT_TYPE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Semantic: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_EFFECT_VARIABLE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_EFFECT_VARIABLE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_EFFECT_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
pub type D3D10_FEATURE_LEVEL1 = i32;
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = 40960i32;
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = 41216i32;
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = 37120i32;
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = 37376i32;
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = 37632i32;
pub type D3D10_FILL_MODE = i32;
pub const D3D10_FILL_WIREFRAME: D3D10_FILL_MODE = 2i32;
pub const D3D10_FILL_SOLID: D3D10_FILL_MODE = 3i32;
pub type D3D10_FILTER = i32;
pub const D3D10_FILTER_MIN_MAG_MIP_POINT: D3D10_FILTER = 0i32;
pub const D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 1i32;
pub const D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 4i32;
pub const D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = 5i32;
pub const D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = 16i32;
pub const D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 17i32;
pub const D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 20i32;
pub const D3D10_FILTER_MIN_MAG_MIP_LINEAR: D3D10_FILTER = 21i32;
pub const D3D10_FILTER_ANISOTROPIC: D3D10_FILTER = 85i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D10_FILTER = 128i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 129i32;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 132i32;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = 133i32;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = 144i32;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 145i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 148i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D10_FILTER = 149i32;
pub const D3D10_FILTER_COMPARISON_ANISOTROPIC: D3D10_FILTER = 213i32;
pub const D3D10_FILTER_TEXT_1BIT: D3D10_FILTER = -2147483648i32;
pub type D3D10_FILTER_TYPE = i32;
pub const D3D10_FILTER_TYPE_POINT: D3D10_FILTER_TYPE = 0i32;
pub const D3D10_FILTER_TYPE_LINEAR: D3D10_FILTER_TYPE = 1i32;
pub const D3D10_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D10_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D10_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub type D3D10_FORMAT_SUPPORT = i32;
pub const D3D10_FORMAT_SUPPORT_BUFFER: D3D10_FORMAT_SUPPORT = 1i32;
pub const D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D10_FORMAT_SUPPORT = 2i32;
pub const D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D10_FORMAT_SUPPORT = 4i32;
pub const D3D10_FORMAT_SUPPORT_SO_BUFFER: D3D10_FORMAT_SUPPORT = 8i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE1D: D3D10_FORMAT_SUPPORT = 16i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE2D: D3D10_FORMAT_SUPPORT = 32i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE3D: D3D10_FORMAT_SUPPORT = 64i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURECUBE: D3D10_FORMAT_SUPPORT = 128i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_LOAD: D3D10_FORMAT_SUPPORT = 256i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE: D3D10_FORMAT_SUPPORT = 512i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D10_FORMAT_SUPPORT = 1024i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D10_FORMAT_SUPPORT = 2048i32;
pub const D3D10_FORMAT_SUPPORT_MIP: D3D10_FORMAT_SUPPORT = 4096i32;
pub const D3D10_FORMAT_SUPPORT_MIP_AUTOGEN: D3D10_FORMAT_SUPPORT = 8192i32;
pub const D3D10_FORMAT_SUPPORT_RENDER_TARGET: D3D10_FORMAT_SUPPORT = 16384i32;
pub const D3D10_FORMAT_SUPPORT_BLENDABLE: D3D10_FORMAT_SUPPORT = 32768i32;
pub const D3D10_FORMAT_SUPPORT_DEPTH_STENCIL: D3D10_FORMAT_SUPPORT = 65536i32;
pub const D3D10_FORMAT_SUPPORT_CPU_LOCKABLE: D3D10_FORMAT_SUPPORT = 131072i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D10_FORMAT_SUPPORT = 262144i32;
pub const D3D10_FORMAT_SUPPORT_DISPLAY: D3D10_FORMAT_SUPPORT = 524288i32;
pub const D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D10_FORMAT_SUPPORT = 1048576i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D10_FORMAT_SUPPORT = 2097152i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D10_FORMAT_SUPPORT = 4194304i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_GATHER: D3D10_FORMAT_SUPPORT = 8388608i32;
pub const D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D10_FORMAT_SUPPORT = 16777216i32;
pub const D3D10_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D10_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D10_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D10_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_VERTICES: u32 = 6u32;
pub const D3D10_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D10_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 64u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
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
pub type D3D10_INPUT_CLASSIFICATION = i32;
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = 0i32;
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for D3D10_INPUT_ELEMENT_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D10_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_INPUT_ELEMENT_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
pub type D3D10_MAP = i32;
pub const D3D10_MAP_READ: D3D10_MAP = 1i32;
pub const D3D10_MAP_WRITE: D3D10_MAP = 2i32;
pub const D3D10_MAP_READ_WRITE: D3D10_MAP = 3i32;
pub const D3D10_MAP_WRITE_DISCARD: D3D10_MAP = 4i32;
pub const D3D10_MAP_WRITE_NO_OVERWRITE: D3D10_MAP = 5i32;
#[repr(C)]
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
pub type D3D10_MAP_FLAG = i32;
pub const D3D10_MAP_FLAG_DO_NOT_WAIT: D3D10_MAP_FLAG = 1048576i32;
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D10_MAX_DEPTH: f32 = 1f32;
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D10_MESSAGE {}
impl ::core::clone::Clone for D3D10_MESSAGE {
    fn clone(&self) -> Self {
        *self
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
pub type D3D10_MESSAGE_CATEGORY = i32;
pub const D3D10_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D10_MESSAGE_CATEGORY = 0i32;
pub const D3D10_MESSAGE_CATEGORY_MISCELLANEOUS: D3D10_MESSAGE_CATEGORY = 1i32;
pub const D3D10_MESSAGE_CATEGORY_INITIALIZATION: D3D10_MESSAGE_CATEGORY = 2i32;
pub const D3D10_MESSAGE_CATEGORY_CLEANUP: D3D10_MESSAGE_CATEGORY = 3i32;
pub const D3D10_MESSAGE_CATEGORY_COMPILATION: D3D10_MESSAGE_CATEGORY = 4i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_CREATION: D3D10_MESSAGE_CATEGORY = 5i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_SETTING: D3D10_MESSAGE_CATEGORY = 6i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_GETTING: D3D10_MESSAGE_CATEGORY = 7i32;
pub const D3D10_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D10_MESSAGE_CATEGORY = 8i32;
pub const D3D10_MESSAGE_CATEGORY_EXECUTION: D3D10_MESSAGE_CATEGORY = 9i32;
pub const D3D10_MESSAGE_CATEGORY_SHADER: D3D10_MESSAGE_CATEGORY = 10i32;
pub type D3D10_MESSAGE_ID = i32;
pub const D3D10_MESSAGE_ID_UNKNOWN: D3D10_MESSAGE_ID = 0i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D10_MESSAGE_ID = 1i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D10_MESSAGE_ID = 2i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 3i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 4i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 5i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 6i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 7i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 8i32;
pub const D3D10_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D10_MESSAGE_ID = 9i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D10_MESSAGE_ID = 10i32;
pub const D3D10_MESSAGE_ID_STRING_FROM_APPLICATION: D3D10_MESSAGE_ID = 11i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_THIS: D3D10_MESSAGE_ID = 12i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D10_MESSAGE_ID = 13i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D10_MESSAGE_ID = 14i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D10_MESSAGE_ID = 15i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D10_MESSAGE_ID = 16i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D10_MESSAGE_ID = 17i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D10_MESSAGE_ID = 18i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D10_MESSAGE_ID = 19i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D10_MESSAGE_ID = 20i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D10_MESSAGE_ID = 21i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D10_MESSAGE_ID = 22i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D10_MESSAGE_ID = 23i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D10_MESSAGE_ID = 24i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D10_MESSAGE_ID = 25i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D10_MESSAGE_ID = 26i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D10_MESSAGE_ID = 27i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D10_MESSAGE_ID = 28i32;
pub const D3D10_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D10_MESSAGE_ID = 29i32;
pub const D3D10_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 30i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 31i32;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 32i32;
pub const D3D10_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 33i32;
pub const D3D10_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 34i32;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 35i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 36i32;
pub const D3D10_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 37i32;
pub const D3D10_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 38i32;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 39i32;
pub const D3D10_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 40i32;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 41i32;
pub const D3D10_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 42i32;
pub const D3D10_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 43i32;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 44i32;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 45i32;
pub const D3D10_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 46i32;
pub const D3D10_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 47i32;
pub const D3D10_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 48i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 49i32;
pub const D3D10_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 50i32;
pub const D3D10_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D10_MESSAGE_ID = 51i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D10_MESSAGE_ID = 52i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D10_MESSAGE_ID = 53i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D10_MESSAGE_ID = 54i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D10_MESSAGE_ID = 55i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D10_MESSAGE_ID = 56i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 57i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D10_MESSAGE_ID = 58i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 59i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 60i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 61i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 62i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 63i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 64i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 65i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 66i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 67i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 68i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 69i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 70i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D10_MESSAGE_ID = 71i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D10_MESSAGE_ID = 72i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D10_MESSAGE_ID = 73i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 74i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 75i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 76i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 77i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 78i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 79i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 80i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 81i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 82i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 83i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 84i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 85i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 86i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 87i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 88i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D10_MESSAGE_ID = 89i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D10_MESSAGE_ID = 90i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 91i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 92i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 93i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 94i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 95i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 96i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 97i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 98i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 99i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 100i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 101i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 102i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 103i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 104i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 105i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D10_MESSAGE_ID = 106i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D10_MESSAGE_ID = 107i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 108i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 109i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 110i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 111i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 112i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 113i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 114i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 115i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 116i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 117i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 118i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 119i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 120i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 121i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 122i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D10_MESSAGE_ID = 123i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D10_MESSAGE_ID = 124i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 125i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 126i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 127i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 128i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 129i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 130i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 131i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 132i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 133i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 134i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 135i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 136i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 137i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 138i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 139i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 140i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 141i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 142i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 143i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 144i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 145i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 146i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 147i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 148i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 149i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D10_MESSAGE_ID = 150i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D10_MESSAGE_ID = 151i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D10_MESSAGE_ID = 152i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D10_MESSAGE_ID = 153i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D10_MESSAGE_ID = 154i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D10_MESSAGE_ID = 155i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D10_MESSAGE_ID = 156i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D10_MESSAGE_ID = 157i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D10_MESSAGE_ID = 158i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D10_MESSAGE_ID = 159i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D10_MESSAGE_ID = 160i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D10_MESSAGE_ID = 161i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D10_MESSAGE_ID = 162i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D10_MESSAGE_ID = 163i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D10_MESSAGE_ID = 164i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 165i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 166i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 167i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 168i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 169i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 170i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D10_MESSAGE_ID = 171i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 172i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 173i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D10_MESSAGE_ID = 174i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D10_MESSAGE_ID = 175i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D10_MESSAGE_ID = 176i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D10_MESSAGE_ID = 177i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D10_MESSAGE_ID = 178i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D10_MESSAGE_ID = 179i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D10_MESSAGE_ID = 180i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D10_MESSAGE_ID = 181i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D10_MESSAGE_ID = 182i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D10_MESSAGE_ID = 183i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D10_MESSAGE_ID = 184i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D10_MESSAGE_ID = 185i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D10_MESSAGE_ID = 186i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D10_MESSAGE_ID = 187i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D10_MESSAGE_ID = 188i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D10_MESSAGE_ID = 189i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D10_MESSAGE_ID = 190i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 191i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 192i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 193i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D10_MESSAGE_ID = 194i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D10_MESSAGE_ID = 195i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D10_MESSAGE_ID = 196i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D10_MESSAGE_ID = 197i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 198i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D10_MESSAGE_ID = 199i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D10_MESSAGE_ID = 200i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D10_MESSAGE_ID = 201i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D10_MESSAGE_ID = 202i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D10_MESSAGE_ID = 203i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D10_MESSAGE_ID = 204i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D10_MESSAGE_ID = 205i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D10_MESSAGE_ID = 206i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D10_MESSAGE_ID = 207i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D10_MESSAGE_ID = 208i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D10_MESSAGE_ID = 209i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 210i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D10_MESSAGE_ID = 211i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D10_MESSAGE_ID = 212i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D10_MESSAGE_ID = 213i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D10_MESSAGE_ID = 214i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D10_MESSAGE_ID = 215i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D10_MESSAGE_ID = 216i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D10_MESSAGE_ID = 217i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D10_MESSAGE_ID = 218i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 219i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D10_MESSAGE_ID = 220i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D10_MESSAGE_ID = 221i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D10_MESSAGE_ID = 222i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D10_MESSAGE_ID = 223i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D10_MESSAGE_ID = 224i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D10_MESSAGE_ID = 225i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D10_MESSAGE_ID = 226i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D10_MESSAGE_ID = 227i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D10_MESSAGE_ID = 228i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D10_MESSAGE_ID = 229i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 230i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D10_MESSAGE_ID = 231i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D10_MESSAGE_ID = 232i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 233i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D10_MESSAGE_ID = 234i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D10_MESSAGE_ID = 235i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D10_MESSAGE_ID = 236i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D10_MESSAGE_ID = 237i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 238i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = 239i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 240i32;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D10_MESSAGE_ID = 241i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = 242i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = 243i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 244i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 245i32;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 246i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 247i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 248i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 249i32;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 250i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 251i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 252i32;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D10_MESSAGE_ID = 253i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 254i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 255i32;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 256i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 257i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 258i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D10_MESSAGE_ID = 259i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D10_MESSAGE_ID = 260i32;
pub const D3D10_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = 261i32;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = 262i32;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D10_MESSAGE_ID = 263i32;
pub const D3D10_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 264i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 265i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 266i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 267i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 268i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 269i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 270i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 271i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 272i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 273i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 274i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D10_MESSAGE_ID = 275i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D10_MESSAGE_ID = 276i32;
pub const D3D10_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D10_MESSAGE_ID = 277i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = 278i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D10_MESSAGE_ID = 279i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D10_MESSAGE_ID = 280i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D10_MESSAGE_ID = 281i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 282i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = 283i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D10_MESSAGE_ID = 284i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 285i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = 286i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = 287i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D10_MESSAGE_ID = 288i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 289i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D10_MESSAGE_ID = 290i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = 291i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D10_MESSAGE_ID = 292i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = 293i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D10_MESSAGE_ID = 294i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 295i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 296i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 297i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 298i32;
pub const D3D10_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 299i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 300i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 301i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 302i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 303i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 304i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 305i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 306i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 307i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 308i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 309i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 310i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 311i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 312i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 313i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 314i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 315i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 316i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 317i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 318i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 319i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 320i32;
pub const D3D10_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = 321i32;
pub const D3D10_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = 322i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D10_MESSAGE_ID = 323i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 324i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 325i32;
pub const D3D10_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D10_MESSAGE_ID = 326i32;
pub const D3D10_MESSAGE_ID_REF_THREADING_MODE: D3D10_MESSAGE_ID = 327i32;
pub const D3D10_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = 328i32;
pub const D3D10_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = 329i32;
pub const D3D10_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D10_MESSAGE_ID = 330i32;
pub const D3D10_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D10_MESSAGE_ID = 331i32;
pub const D3D10_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D10_MESSAGE_ID = 332i32;
pub const D3D10_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D10_MESSAGE_ID = 333i32;
pub const D3D10_MESSAGE_ID_REF_INFO: D3D10_MESSAGE_ID = 334i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 335i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 336i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 337i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = 338i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = 339i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 340i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D10_MESSAGE_ID = 341i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D10_MESSAGE_ID = 342i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D10_MESSAGE_ID = 343i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D10_MESSAGE_ID = 344i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D10_MESSAGE_ID = 345i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D10_MESSAGE_ID = 346i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D10_MESSAGE_ID = 347i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 348i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D10_MESSAGE_ID = 349i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 350i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 351i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D10_MESSAGE_ID = 352i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D10_MESSAGE_ID = 353i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D10_MESSAGE_ID = 354i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D10_MESSAGE_ID = 355i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 356i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 357i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = 358i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 359i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D10_MESSAGE_ID = 360i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D10_MESSAGE_ID = 361i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D10_MESSAGE_ID = 362i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D10_MESSAGE_ID = 363i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D10_MESSAGE_ID = 364i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D10_MESSAGE_ID = 365i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 366i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D10_MESSAGE_ID = 367i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 368i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 369i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D10_MESSAGE_ID = 370i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = 371i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D10_MESSAGE_ID = 372i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = 373i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D10_MESSAGE_ID = 374i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D10_MESSAGE_ID = 375i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D10_MESSAGE_ID = 376i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D10_MESSAGE_ID = 377i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D10_MESSAGE_ID = 378i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D10_MESSAGE_ID = 379i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D10_MESSAGE_ID = 380i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 381i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 382i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D10_MESSAGE_ID = 383i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D10_MESSAGE_ID = 384i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = 385i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = 386i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D10_MESSAGE_ID = 387i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D10_MESSAGE_ID = 388i32;
pub const D3D10_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 389i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D10_MESSAGE_ID = 390i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = 391i32;
pub const D3D10_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D10_MESSAGE_ID = 392i32;
pub const D3D10_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D10_MESSAGE_ID = 393i32;
pub const D3D10_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 394i32;
pub const D3D10_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 395i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = 396i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D10_MESSAGE_ID = 397i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = 398i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 399i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D10_MESSAGE_ID = 400i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D10_MESSAGE_ID = 401i32;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = 402i32;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = 403i32;
pub const D3D10_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D10_MESSAGE_ID = 404i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D10_MESSAGE_ID = 405i32;
pub const D3D10_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D10_MESSAGE_ID = 406i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D10_MESSAGE_ID = 407i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = 408i32;
pub const D3D10_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D10_MESSAGE_ID = 409i32;
pub const D3D10_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = 410i32;
pub const D3D10_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D10_MESSAGE_ID = 411i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D10_MESSAGE_ID = 412i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D10_MESSAGE_ID = 413i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D10_MESSAGE_ID = 414i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = 415i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D10_MESSAGE_ID = 416i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D10_MESSAGE_ID = 417i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D10_MESSAGE_ID = 418i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D10_MESSAGE_ID = 419i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D10_MESSAGE_ID = 420i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D10_MESSAGE_ID = 421i32;
pub const D3D10_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D10_MESSAGE_ID = 422i32;
pub const D3D10_MESSAGE_ID_LIVE_BUFFER: D3D10_MESSAGE_ID = 423i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE1D: D3D10_MESSAGE_ID = 424i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE2D: D3D10_MESSAGE_ID = 425i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE3D: D3D10_MESSAGE_ID = 426i32;
pub const D3D10_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D10_MESSAGE_ID = 427i32;
pub const D3D10_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D10_MESSAGE_ID = 428i32;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D10_MESSAGE_ID = 429i32;
pub const D3D10_MESSAGE_ID_LIVE_VERTEXSHADER: D3D10_MESSAGE_ID = 430i32;
pub const D3D10_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D10_MESSAGE_ID = 431i32;
pub const D3D10_MESSAGE_ID_LIVE_PIXELSHADER: D3D10_MESSAGE_ID = 432i32;
pub const D3D10_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D10_MESSAGE_ID = 433i32;
pub const D3D10_MESSAGE_ID_LIVE_SAMPLER: D3D10_MESSAGE_ID = 434i32;
pub const D3D10_MESSAGE_ID_LIVE_BLENDSTATE: D3D10_MESSAGE_ID = 435i32;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D10_MESSAGE_ID = 436i32;
pub const D3D10_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D10_MESSAGE_ID = 437i32;
pub const D3D10_MESSAGE_ID_LIVE_QUERY: D3D10_MESSAGE_ID = 438i32;
pub const D3D10_MESSAGE_ID_LIVE_PREDICATE: D3D10_MESSAGE_ID = 439i32;
pub const D3D10_MESSAGE_ID_LIVE_COUNTER: D3D10_MESSAGE_ID = 440i32;
pub const D3D10_MESSAGE_ID_LIVE_DEVICE: D3D10_MESSAGE_ID = 441i32;
pub const D3D10_MESSAGE_ID_LIVE_SWAPCHAIN: D3D10_MESSAGE_ID = 442i32;
pub const D3D10_MESSAGE_ID_D3D10_MESSAGES_END: D3D10_MESSAGE_ID = 443i32;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D10_MESSAGE_ID = 1048576i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D10_MESSAGE_ID = 1048577i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048578i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D10_MESSAGE_ID = 1048579i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D10_MESSAGE_ID = 1048580i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D10_MESSAGE_ID = 1048581i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048582i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = 1048583i32;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = 1048584i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D10_MESSAGE_ID = 1048585i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D10_MESSAGE_ID = 1048586i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D10_MESSAGE_ID = 1048587i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D10_MESSAGE_ID = 1048588i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048589i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D10_MESSAGE_ID = 1048590i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D10_MESSAGE_ID = 1048591i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D10_MESSAGE_ID = 1048592i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D10_MESSAGE_ID = 1048593i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D10_MESSAGE_ID = 1048594i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D10_MESSAGE_ID = 1048595i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D10_MESSAGE_ID = 1048596i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D10_MESSAGE_ID = 1048597i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D10_MESSAGE_ID = 1048598i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D10_MESSAGE_ID = 1048599i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D10_MESSAGE_ID = 1048600i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D10_MESSAGE_ID = 1048601i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D10_MESSAGE_ID = 1048602i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D10_MESSAGE_ID = 1048603i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D10_MESSAGE_ID = 1048604i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D10_MESSAGE_ID = 1048605i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D10_MESSAGE_ID = 1048606i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D10_MESSAGE_ID = 1048607i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D10_MESSAGE_ID = 1048608i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D10_MESSAGE_ID = 1048609i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048610i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048611i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D10_MESSAGE_ID = 1048612i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D10_MESSAGE_ID = 1048613i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D10_MESSAGE_ID = 1048614i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D10_MESSAGE_ID = 1048615i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D10_MESSAGE_ID = 1048616i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D10_MESSAGE_ID = 1048617i32;
pub const D3D10_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048618i32;
pub const D3D10_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048619i32;
pub const D3D10_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048620i32;
pub const D3D10_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048621i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D10_MESSAGE_ID = 1048622i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D10_MESSAGE_ID = 1048623i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048624i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D10_MESSAGE_ID = 1048625i32;
pub const D3D10_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048626i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D10_MESSAGE_ID = 1048627i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D10_MESSAGE_ID = 1048628i32;
pub const D3D10_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D10_MESSAGE_ID = 1048629i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D10_MESSAGE_ID = 1048630i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D10_MESSAGE_ID = 1048631i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D10_MESSAGE_ID = 1048632i32;
pub const D3D10_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D10_MESSAGE_ID = 1048633i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D10_MESSAGE_ID = 1048634i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048635i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D10_MESSAGE_ID = 1048636i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D10_MESSAGE_ID = 1048637i32;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D10_MESSAGE_ID = 1048638i32;
pub type D3D10_MESSAGE_SEVERITY = i32;
pub const D3D10_MESSAGE_SEVERITY_CORRUPTION: D3D10_MESSAGE_SEVERITY = 0i32;
pub const D3D10_MESSAGE_SEVERITY_ERROR: D3D10_MESSAGE_SEVERITY = 1i32;
pub const D3D10_MESSAGE_SEVERITY_WARNING: D3D10_MESSAGE_SEVERITY = 2i32;
pub const D3D10_MESSAGE_SEVERITY_INFO: D3D10_MESSAGE_SEVERITY = 3i32;
pub const D3D10_MESSAGE_SEVERITY_MESSAGE: D3D10_MESSAGE_SEVERITY = 4i32;
pub const D3D10_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_MIN_DEPTH: f32 = 0f32;
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D10_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D10_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6u32;
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_PASS_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_PASS_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_PASS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_PASS_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_PASS_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_PASS_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_PASS_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: ::core::option::Option<ID3D10EffectShaderVariable>,
    pub ShaderIndex: u32,
}
impl ::core::clone::Clone for D3D10_PASS_SHADER_DESC {
    fn clone(&self) -> Self {
        Self { pShaderVariable: self.pShaderVariable.clone(), ShaderIndex: self.ShaderIndex }
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
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13u32;
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub type D3D10_QUERY = i32;
pub const D3D10_QUERY_EVENT: D3D10_QUERY = 0i32;
pub const D3D10_QUERY_OCCLUSION: D3D10_QUERY = 1i32;
pub const D3D10_QUERY_TIMESTAMP: D3D10_QUERY = 2i32;
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: D3D10_QUERY = 3i32;
pub const D3D10_QUERY_PIPELINE_STATISTICS: D3D10_QUERY = 4i32;
pub const D3D10_QUERY_OCCLUSION_PREDICATE: D3D10_QUERY = 5i32;
pub const D3D10_QUERY_SO_STATISTICS: D3D10_QUERY = 6i32;
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: D3D10_QUERY = 7i32;
#[repr(C)]
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
pub type D3D10_QUERY_MISC_FLAG = i32;
pub const D3D10_QUERY_MISC_PREDICATEHINT: D3D10_QUERY_MISC_FLAG = 1i32;
pub type D3D10_RAISE_FLAG = i32;
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D10_RAISE_FLAG = 1i32;
#[repr(C)]
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
#[repr(C)]
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
pub const D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_REQ_MIP_LEVELS: u32 = 14u32;
pub const D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 8192u32;
pub const D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES: u32 = 128u32;
pub const D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP: u32 = 20u32;
pub const D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE1D_U_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D10_REQ_TEXTURECUBE_DIMENSION: u32 = 8192u32;
pub const D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub type D3D10_RESOURCE_DIMENSION = i32;
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: D3D10_RESOURCE_DIMENSION = 0i32;
pub const D3D10_RESOURCE_DIMENSION_BUFFER: D3D10_RESOURCE_DIMENSION = 1i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: D3D10_RESOURCE_DIMENSION = 2i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: D3D10_RESOURCE_DIMENSION = 3i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: D3D10_RESOURCE_DIMENSION = 4i32;
pub type D3D10_RESOURCE_MISC_FLAG = i32;
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: D3D10_RESOURCE_MISC_FLAG = 1i32;
pub const D3D10_RESOURCE_MISC_SHARED: D3D10_RESOURCE_MISC_FLAG = 2i32;
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: D3D10_RESOURCE_MISC_FLAG = 4i32;
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D10_RESOURCE_MISC_FLAG = 16i32;
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: D3D10_RESOURCE_MISC_FLAG = 32i32;
pub type D3D10_RTV_DIMENSION = i32;
pub const D3D10_RTV_DIMENSION_UNKNOWN: D3D10_RTV_DIMENSION = 0i32;
pub const D3D10_RTV_DIMENSION_BUFFER: D3D10_RTV_DIMENSION = 1i32;
pub const D3D10_RTV_DIMENSION_TEXTURE1D: D3D10_RTV_DIMENSION = 2i32;
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: D3D10_RTV_DIMENSION = 3i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2D: D3D10_RTV_DIMENSION = 4i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: D3D10_RTV_DIMENSION = 5i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: D3D10_RTV_DIMENSION = 6i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D10_RTV_DIMENSION = 7i32;
pub const D3D10_RTV_DIMENSION_TEXTURE3D: D3D10_RTV_DIMENSION = 8i32;
#[repr(C)]
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
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
pub const D3D10_SDK_VERSION: u32 = 29u32;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_BUFFER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_BUFFER_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_BUFFER_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D10_SHADER_BUFFER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
#[repr(C)]
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
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[repr(C)]
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
pub type D3D10_SHADER_DEBUG_REGTYPE = i32;
pub const D3D10_SHADER_DEBUG_REG_INPUT: D3D10_SHADER_DEBUG_REGTYPE = 0i32;
pub const D3D10_SHADER_DEBUG_REG_OUTPUT: D3D10_SHADER_DEBUG_REGTYPE = 1i32;
pub const D3D10_SHADER_DEBUG_REG_CBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 2i32;
pub const D3D10_SHADER_DEBUG_REG_TBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 3i32;
pub const D3D10_SHADER_DEBUG_REG_TEMP: D3D10_SHADER_DEBUG_REGTYPE = 4i32;
pub const D3D10_SHADER_DEBUG_REG_TEMPARRAY: D3D10_SHADER_DEBUG_REGTYPE = 5i32;
pub const D3D10_SHADER_DEBUG_REG_TEXTURE: D3D10_SHADER_DEBUG_REGTYPE = 6i32;
pub const D3D10_SHADER_DEBUG_REG_SAMPLER: D3D10_SHADER_DEBUG_REGTYPE = 7i32;
pub const D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 8i32;
pub const D3D10_SHADER_DEBUG_REG_LITERAL: D3D10_SHADER_DEBUG_REGTYPE = 9i32;
pub const D3D10_SHADER_DEBUG_REG_UNUSED: D3D10_SHADER_DEBUG_REGTYPE = 10i32;
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: D3D10_SHADER_DEBUG_REGTYPE = 11i32;
pub const D3D11_SHADER_DEBUG_REG_UAV: D3D10_SHADER_DEBUG_REGTYPE = 12i32;
pub const D3D10_SHADER_DEBUG_REG_FORCE_DWORD: D3D10_SHADER_DEBUG_REGTYPE = 2147483647i32;
pub type D3D10_SHADER_DEBUG_SCOPETYPE = i32;
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: D3D10_SHADER_DEBUG_SCOPETYPE = 0i32;
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = 1i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: D3D10_SHADER_DEBUG_SCOPETYPE = 2i32;
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: D3D10_SHADER_DEBUG_SCOPETYPE = 3i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: D3D10_SHADER_DEBUG_SCOPETYPE = 4i32;
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = 5i32;
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: D3D10_SHADER_DEBUG_SCOPETYPE = 6i32;
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: D3D10_SHADER_DEBUG_SCOPETYPE = 7i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FORCE_DWORD: D3D10_SHADER_DEBUG_SCOPETYPE = 2147483647i32;
#[repr(C)]
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
pub type D3D10_SHADER_DEBUG_VARTYPE = i32;
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = 0i32;
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: D3D10_SHADER_DEBUG_VARTYPE = 1i32;
pub const D3D10_SHADER_DEBUG_VAR_FORCE_DWORD: D3D10_SHADER_DEBUG_VARTYPE = 2147483647i32;
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D10_SHADER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D10_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D10_SHADER_INPUT_BIND_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D10_SHADER_INPUT_BIND_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_INPUT_BIND_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0u32;
pub const D3D10_SHADER_NO_PRESHADER: u32 = 256u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: u32 = 16384u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: u32 = 0u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: u32 = 32768u32;
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
pub const D3D10_SHADER_PARTIAL_PRECISION: u32 = 32u32;
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: u32 = 1024u32;
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[repr(C)]
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
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_SHADER_VARIABLE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SHADER_VARIABLE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
unsafe impl ::windows::core::Abi for D3D10_SIGNATURE_PARAMETER_DESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D10_SIGNATURE_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SIGNATURE_PARAMETER_DESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SO_DECLARATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_SO_DECLARATION_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_SO_DECLARATION_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SO_DECLARATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D10_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D10_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D10_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub type D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = i32;
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -1i32;
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2i32;
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
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
pub type D3D10_STENCIL_OP = i32;
pub const D3D10_STENCIL_OP_KEEP: D3D10_STENCIL_OP = 1i32;
pub const D3D10_STENCIL_OP_ZERO: D3D10_STENCIL_OP = 2i32;
pub const D3D10_STENCIL_OP_REPLACE: D3D10_STENCIL_OP = 3i32;
pub const D3D10_STENCIL_OP_INCR_SAT: D3D10_STENCIL_OP = 4i32;
pub const D3D10_STENCIL_OP_DECR_SAT: D3D10_STENCIL_OP = 5i32;
pub const D3D10_STENCIL_OP_INVERT: D3D10_STENCIL_OP = 6i32;
pub const D3D10_STENCIL_OP_INCR: D3D10_STENCIL_OP = 7i32;
pub const D3D10_STENCIL_OP_DECR: D3D10_STENCIL_OP = 8i32;
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *mut ::core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl ::core::marker::Copy for D3D10_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D10_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
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
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_TECHNIQUE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_TECHNIQUE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for D3D10_TECHNIQUE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_TECHNIQUE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D10_TECHNIQUE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_TECHNIQUE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_DSV {}
impl ::core::clone::Clone for D3D10_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_RTV {}
impl ::core::clone::Clone for D3D10_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_DSV {}
impl ::core::clone::Clone for D3D10_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
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
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_RTV {}
impl ::core::clone::Clone for D3D10_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
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
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
#[repr(C)]
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
pub type D3D10_TEXTURECUBE_FACE = i32;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_X: D3D10_TEXTURECUBE_FACE = 0i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_X: D3D10_TEXTURECUBE_FACE = 1i32;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Y: D3D10_TEXTURECUBE_FACE = 2i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Y: D3D10_TEXTURECUBE_FACE = 3i32;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Z: D3D10_TEXTURECUBE_FACE = 4i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Z: D3D10_TEXTURECUBE_FACE = 5i32;
pub type D3D10_TEXTURE_ADDRESS_MODE = i32;
pub const D3D10_TEXTURE_ADDRESS_WRAP: D3D10_TEXTURE_ADDRESS_MODE = 1i32;
pub const D3D10_TEXTURE_ADDRESS_MIRROR: D3D10_TEXTURE_ADDRESS_MODE = 2i32;
pub const D3D10_TEXTURE_ADDRESS_CLAMP: D3D10_TEXTURE_ADDRESS_MODE = 3i32;
pub const D3D10_TEXTURE_ADDRESS_BORDER: D3D10_TEXTURE_ADDRESS_MODE = 4i32;
pub const D3D10_TEXTURE_ADDRESS_MIRROR_ONCE: D3D10_TEXTURE_ADDRESS_MODE = 5i32;
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub type D3D10_USAGE = i32;
pub const D3D10_USAGE_DEFAULT: D3D10_USAGE = 0i32;
pub const D3D10_USAGE_IMMUTABLE: D3D10_USAGE = 1i32;
pub const D3D10_USAGE_DYNAMIC: D3D10_USAGE = 2i32;
pub const D3D10_USAGE_STAGING: D3D10_USAGE = 3i32;
#[repr(C)]
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
pub const D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D10_VIEWPORT_BOUNDS_MAX: u32 = 16383u32;
pub const D3D10_VIEWPORT_BOUNDS_MIN: i32 = -16384i32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_OUTPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D_MAJOR_VERSION: u32 = 10u32;
pub const D3D_MINOR_VERSION: u32 = 0u32;
pub const D3D_SPEC_DATE_DAY: u32 = 8u32;
pub const D3D_SPEC_DATE_MONTH: u32 = 8u32;
pub const D3D_SPEC_DATE_YEAR: u32 = 2006u32;
pub const D3D_SPEC_VERSION: f64 = 1.050005f64;
pub const DXGI_DEBUG_D3D10: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243b4c52_3606_4d3a_99d7_a7e7b33ed706);
pub const GUID_DeviceType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
#[repr(transparent)]
pub struct ID3D10Asynchronous(::windows::core::IUnknown);
impl ID3D10Asynchronous {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(datasize), ::core::mem::transmute(getdataflags)).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10Asynchronous> for ID3D10DeviceChild {
    fn from(value: ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Asynchronous> for ID3D10DeviceChild {
    fn from(value: &ID3D10Asynchronous) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Asynchronous {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Asynchronous {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Asynchronous> for ::windows::core::IUnknown {
    fn from(value: ID3D10Asynchronous) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Asynchronous> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Asynchronous) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Asynchronous {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Asynchronous {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Asynchronous {
    type Vtable = ID3D10AsynchronousVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0d_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10AsynchronousVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
);
#[repr(transparent)]
pub struct ID3D10BlendState(::windows::core::IUnknown);
impl ID3D10BlendState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10BlendState> for ID3D10DeviceChild {
    fn from(value: ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState> for ID3D10DeviceChild {
    fn from(value: &ID3D10BlendState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10BlendState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10BlendState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10BlendState> for ::windows::core::IUnknown {
    fn from(value: ID3D10BlendState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10BlendState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10BlendState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10BlendState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10BlendState {
    type Vtable = ID3D10BlendStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedad8d19_8a35_4d6d_8566_2ea276cde161);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10BlendState1(::windows::core::IUnknown);
impl ID3D10BlendState1 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ID3D10BlendState {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ID3D10BlendState {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10BlendState> for ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10BlendState> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10BlendState> for &ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10BlendState> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ID3D10DeviceChild {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ID3D10DeviceChild {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10BlendState1> for ::windows::core::IUnknown {
    fn from(value: ID3D10BlendState1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10BlendState1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10BlendState1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10BlendState1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10BlendState1 {
    type Vtable = ID3D10BlendState1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedad8d99_8a35_4d6d_8566_2ea276cde161);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Buffer(::windows::core::IUnknown);
impl ID3D10Buffer {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority))
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(maptype), ::core::mem::transmute(mapflags), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn Unmap(&self) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Buffer> for ID3D10Resource {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ID3D10Resource {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for &ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Buffer> for ID3D10DeviceChild {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ID3D10DeviceChild {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Buffer> for ::windows::core::IUnknown {
    fn from(value: ID3D10Buffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Buffer> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Buffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Buffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Buffer {
    type Vtable = ID3D10BufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c02_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC),
);
#[repr(transparent)]
pub struct ID3D10Counter(::windows::core::IUnknown);
impl ID3D10Counter {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(datasize), ::core::mem::transmute(getdataflags)).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Counter> for ID3D10Asynchronous {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ID3D10Asynchronous {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for &ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Counter> for ID3D10DeviceChild {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ID3D10DeviceChild {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Counter> for ::windows::core::IUnknown {
    fn from(value: ID3D10Counter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Counter> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Counter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Counter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Counter {
    type Vtable = ID3D10CounterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c11_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10CounterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC),
);
#[repr(transparent)]
pub struct ID3D10Debug(::windows::core::IUnknown);
impl ID3D10Debug {
    pub unsafe fn SetFeatureMask(&self, mask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(mask)).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(milliseconds)).ok()
    }
    pub unsafe fn GetPresentPerRenderOpDelay(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGISwapChain>>(&self, pswapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pswapchain.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSwapChain(&self) -> ::windows::core::Result<super::Dxgi::IDXGISwapChain> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Dxgi::IDXGISwapChain>(result__)
    }
    pub unsafe fn Validate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ID3D10Debug> for ::windows::core::IUnknown {
    fn from(value: ID3D10Debug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Debug> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Debug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Debug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Debug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Debug {
    type Vtable = ID3D10DebugVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e01_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DebugVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pswapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10DepthStencilState(::windows::core::IUnknown);
impl ID3D10DepthStencilState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10DepthStencilState> for ID3D10DeviceChild {
    fn from(value: ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilState> for ID3D10DeviceChild {
    fn from(value: &ID3D10DepthStencilState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10DepthStencilState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10DepthStencilState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10DepthStencilState> for ::windows::core::IUnknown {
    fn from(value: ID3D10DepthStencilState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DepthStencilState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10DepthStencilState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10DepthStencilState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10DepthStencilState {
    type Vtable = ID3D10DepthStencilStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b4b1cc8_a4ad_41f8_8322_ca86fc3ec675);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10DepthStencilView(::windows::core::IUnknown);
impl ID3D10DepthStencilView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresource))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ID3D10View {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ID3D10View {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for &ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ID3D10DeviceChild {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ID3D10DeviceChild {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10DepthStencilView> for ::windows::core::IUnknown {
    fn from(value: ID3D10DepthStencilView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DepthStencilView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DepthStencilView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10DepthStencilView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10DepthStencilView {
    type Vtable = ID3D10DepthStencilViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c09_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Device(::windows::core::IUnknown);
impl ID3D10Device {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn PSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10PixelShader>>(&self, ppixelshader: Param0) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ppixelshader.into_param().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn VSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10VertexShader>>(&self, pvertexshader: Param0) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvertexshader.into_param().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(startindexlocation), ::core::mem::transmute(basevertexlocation))
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexcount), ::core::mem::transmute(startvertexlocation))
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn IASetInputLayout<'a, Param0: ::windows::core::IntoParam<'a, ID3D10InputLayout>>(&self, pinputlayout: Param0) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pinputlayout.into_param().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Buffer>>(&self, pindexbuffer: Param0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pindexbuffer.into_param().abi(), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcountperinstance), ::core::mem::transmute(instancecount), ::core::mem::transmute(startindexlocation), ::core::mem::transmute(basevertexlocation), ::core::mem::transmute(startinstancelocation))
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexcountperinstance), ::core::mem::transmute(instancecount), ::core::mem::transmute(startvertexlocation), ::core::mem::transmute(startinstancelocation))
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn GSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10GeometryShader>>(&self, pshader: Param0) {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pshader.into_param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(topology))
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Predicate>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppredicate: Param0, predicatevalue: Param1) {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppredicate.into_param().abi(), predicatevalue.into_param().abi())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn OMSetRenderTargets<'a, Param2: ::windows::core::IntoParam<'a, ID3D10DepthStencilView>>(&self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D10RenderTargetView>, pdepthstencilview: Param2) {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviews), ::core::mem::transmute(pprendertargetviews), pdepthstencilview.into_param().abi())
    }
    pub unsafe fn OMSetBlendState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10BlendState>>(&self, pblendstate: Param0, blendfactor: *const f32, samplemask: u32) {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pblendstate.into_param().abi(), ::core::mem::transmute(blendfactor), ::core::mem::transmute(samplemask))
    }
    pub unsafe fn OMSetDepthStencilState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10DepthStencilState>>(&self, pdepthstencilstate: Param0, stencilref: u32) {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pdepthstencilstate.into_param().abi(), ::core::mem::transmute(stencilref))
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn RSSetState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10RasterizerState>>(&self, prasterizerstate: Param0) {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prasterizerstate.into_param().abi())
    }
    pub unsafe fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn CopySubresourceRegion<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param5: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: Param5, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), ::core::mem::transmute(dstx), ::core::mem::transmute(dsty), ::core::mem::transmute(dstz), psrcresource.into_param().abi(), ::core::mem::transmute(srcsubresource), ::core::mem::transmute(psrcbox))
    }
    pub unsafe fn CopyResource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param1: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, psrcresource: Param1) {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), psrcresource.into_param().abi())
    }
    pub unsafe fn UpdateSubresource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), ::core::mem::transmute(pdstbox), ::core::mem::transmute(psrcdata), ::core::mem::transmute(srcrowpitch), ::core::mem::transmute(srcdepthpitch))
    }
    pub unsafe fn ClearRenderTargetView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10RenderTargetView>>(&self, prendertargetview: Param0, colorrgba: *const f32) {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), prendertargetview.into_param().abi(), ::core::mem::transmute(colorrgba))
    }
    pub unsafe fn ClearDepthStencilView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10DepthStencilView>>(&self, pdepthstencilview: Param0, clearflags: u32, depth: f32, stencil: u8) {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), pdepthstencilview.into_param().abi(), ::core::mem::transmute(clearflags), ::core::mem::transmute(depth), ::core::mem::transmute(stencil))
    }
    pub unsafe fn GenerateMips<'a, Param0: ::windows::core::IntoParam<'a, ID3D10ShaderResourceView>>(&self, pshaderresourceview: Param0) {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pshaderresourceview.into_param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param2: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, psrcresource: Param2, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), psrcresource.into_param().abi(), ::core::mem::transmute(srcsubresource), ::core::mem::transmute(format))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>) {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppixelshader))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>) {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvertexshader))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>) {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppinputlayout))
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindexbuffer), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppgeometryshader))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptopology))
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL) {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppredicate), ::core::mem::transmute(ppredicatevalue))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>) {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviews), ::core::mem::transmute(pprendertargetviews), ::core::mem::transmute(ppdepthstencilview))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32) {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppblendstate), ::core::mem::transmute(blendfactor), ::core::mem::transmute(psamplemask))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32) {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdepthstencilstate), ::core::mem::transmute(pstencilref))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>) {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprasterizerstate))
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(raiseflags)).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Buffer>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture1D>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture2D>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture3D>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RenderTargetView>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilView>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10InputLayout> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinputelementdescs), ::core::mem::transmute(numelements), ::core::mem::transmute(pshaderbytecodewithinputsignature), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10InputLayout>(result__)
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10GeometryShader>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(psodeclaration), ::core::mem::transmute(numentries), ::core::mem::transmute(outputstreamstride), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10PixelShader>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10BlendState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdepthstencildesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(prasterizerdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RasterizerState>(result__)
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(psamplerdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(pquerydesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Query>(result__)
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppredicatedesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Predicate>(result__)
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcounterdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Counter>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(samplecount), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO) {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcounterinfo))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(ptype), ::core::mem::transmute(pactivecounters), ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength)).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(returnedinterface), ::core::mem::transmute(ppresource)).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height))
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32) {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight))
    }
}
impl ::core::convert::From<ID3D10Device> for ::windows::core::IUnknown {
    fn from(value: ID3D10Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Device) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Device {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Device {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Device {
    type Vtable = ID3D10DeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0f_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D10_BOX),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr, poffsets: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32),
);
#[repr(transparent)]
pub struct ID3D10Device1(::windows::core::IUnknown);
impl ID3D10Device1 {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn PSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10PixelShader>>(&self, ppixelshader: Param0) {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ppixelshader.into_param().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn VSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10VertexShader>>(&self, pvertexshader: Param0) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvertexshader.into_param().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcount), ::core::mem::transmute(startindexlocation), ::core::mem::transmute(basevertexlocation))
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexcount), ::core::mem::transmute(startvertexlocation))
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn IASetInputLayout<'a, Param0: ::windows::core::IntoParam<'a, ID3D10InputLayout>>(&self, pinputlayout: Param0) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pinputlayout.into_param().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Buffer>>(&self, pindexbuffer: Param0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pindexbuffer.into_param().abi(), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexcountperinstance), ::core::mem::transmute(instancecount), ::core::mem::transmute(startindexlocation), ::core::mem::transmute(basevertexlocation), ::core::mem::transmute(startinstancelocation))
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(vertexcountperinstance), ::core::mem::transmute(instancecount), ::core::mem::transmute(startvertexlocation), ::core::mem::transmute(startinstancelocation))
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn GSSetShader<'a, Param0: ::windows::core::IntoParam<'a, ID3D10GeometryShader>>(&self, pshader: Param0) {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pshader.into_param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(topology))
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Predicate>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppredicate: Param0, predicatevalue: Param1) {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppredicate.into_param().abi(), predicatevalue.into_param().abi())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn OMSetRenderTargets<'a, Param2: ::windows::core::IntoParam<'a, ID3D10DepthStencilView>>(&self, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D10RenderTargetView>, pdepthstencilview: Param2) {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviews), ::core::mem::transmute(pprendertargetviews), pdepthstencilview.into_param().abi())
    }
    pub unsafe fn OMSetBlendState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10BlendState>>(&self, pblendstate: Param0, blendfactor: *const f32, samplemask: u32) {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pblendstate.into_param().abi(), ::core::mem::transmute(blendfactor), ::core::mem::transmute(samplemask))
    }
    pub unsafe fn OMSetDepthStencilState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10DepthStencilState>>(&self, pdepthstencilstate: Param0, stencilref: u32) {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pdepthstencilstate.into_param().abi(), ::core::mem::transmute(stencilref))
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32) {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn RSSetState<'a, Param0: ::windows::core::IntoParam<'a, ID3D10RasterizerState>>(&self, prasterizerstate: Param0) {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prasterizerstate.into_param().abi())
    }
    pub unsafe fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn CopySubresourceRegion<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param5: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: Param5, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), ::core::mem::transmute(dstx), ::core::mem::transmute(dsty), ::core::mem::transmute(dstz), psrcresource.into_param().abi(), ::core::mem::transmute(srcsubresource), ::core::mem::transmute(psrcbox))
    }
    pub unsafe fn CopyResource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param1: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, psrcresource: Param1) {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), psrcresource.into_param().abi())
    }
    pub unsafe fn UpdateSubresource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), ::core::mem::transmute(pdstbox), ::core::mem::transmute(psrcdata), ::core::mem::transmute(srcrowpitch), ::core::mem::transmute(srcdepthpitch))
    }
    pub unsafe fn ClearRenderTargetView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10RenderTargetView>>(&self, prendertargetview: Param0, colorrgba: *const f32) {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), prendertargetview.into_param().abi(), ::core::mem::transmute(colorrgba))
    }
    pub unsafe fn ClearDepthStencilView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10DepthStencilView>>(&self, pdepthstencilview: Param0, clearflags: u32, depth: f32, stencil: u8) {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), pdepthstencilview.into_param().abi(), ::core::mem::transmute(clearflags), ::core::mem::transmute(depth), ::core::mem::transmute(stencil))
    }
    pub unsafe fn GenerateMips<'a, Param0: ::windows::core::IntoParam<'a, ID3D10ShaderResourceView>>(&self, pshaderresourceview: Param0) {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pshaderresourceview.into_param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>, Param2: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, pdstresource: Param0, dstsubresource: u32, psrcresource: Param2, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), pdstresource.into_param().abi(), ::core::mem::transmute(dstsubresource), psrcresource.into_param().abi(), ::core::mem::transmute(srcsubresource), ::core::mem::transmute(format))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>) {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppixelshader))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>) {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvertexshader))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn IAGetInputLayout(&self, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>) {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppinputlayout))
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppvertexbuffers), ::core::mem::transmute(pstrides), ::core::mem::transmute(poffsets))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindexbuffer), ::core::mem::transmute(format), ::core::mem::transmute(offset))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>) {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppconstantbuffers))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppgeometryshader))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptopology))
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL) {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppredicate), ::core::mem::transmute(ppredicatevalue))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>) {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numviews), ::core::mem::transmute(ppshaderresourceviews))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>) {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(startslot), ::core::mem::transmute(numsamplers), ::core::mem::transmute(ppsamplers))
    }
    pub unsafe fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>) {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviews), ::core::mem::transmute(pprendertargetviews), ::core::mem::transmute(ppdepthstencilview))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32) {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppblendstate), ::core::mem::transmute(blendfactor), ::core::mem::transmute(psamplemask))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32) {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdepthstencilstate), ::core::mem::transmute(pstencilref))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32) {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(numbuffers), ::core::mem::transmute(ppsotargets), ::core::mem::transmute(poffsets))
    }
    pub unsafe fn RSGetState(&self, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>) {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprasterizerstate))
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(numviewports), ::core::mem::transmute(pviewports))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(numrects), ::core::mem::transmute(prects))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(raiseflags)).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Buffer>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture1D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture1D>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture2D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture2D>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows::core::Result<ID3D10Texture3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(pinitialdata), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Texture3D>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC) -> ::windows::core::Result<ID3D10RenderTargetView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RenderTargetView>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC) -> ::windows::core::Result<ID3D10DepthStencilView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilView>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10InputLayout> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinputelementdescs), ::core::mem::transmute(numelements), ::core::mem::transmute(pshaderbytecodewithinputsignature), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10InputLayout>(result__)
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10GeometryShader>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(psodeclaration), ::core::mem::transmute(numentries), ::core::mem::transmute(outputstreamstride), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshaderbytecode), ::core::mem::transmute(bytecodelength), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10PixelShader>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10BlendState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdepthstencildesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(prasterizerdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RasterizerState>(result__)
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(psamplerdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Query> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(pquerydesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Query>(result__)
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC) -> ::windows::core::Result<ID3D10Predicate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppredicatedesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Predicate>(result__)
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC) -> ::windows::core::Result<ID3D10Counter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcounterdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Counter>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(samplecount), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO) {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcounterinfo))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc), ::core::mem::transmute(ptype), ::core::mem::transmute(pactivecounters), ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength)).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hresource: Param0, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), hresource.into_param().abi(), ::core::mem::transmute(returnedinterface), ::core::mem::transmute(ppresource)).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height))
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32) {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Resource>>(&self, presource: Param0, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1) -> ::windows::core::Result<ID3D10ShaderResourceView1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), presource.into_param().abi(), ::core::mem::transmute(pdesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderResourceView1>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1) -> ::windows::core::Result<ID3D10BlendState1> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(pblendstatedesc), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10BlendState1>(result__)
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).100)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10Device1> for ID3D10Device {
    fn from(value: ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device1> for ID3D10Device {
    fn from(value: &ID3D10Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Device> for ID3D10Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Device> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Device> for &ID3D10Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Device> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Device1> for ::windows::core::IUnknown {
    fn from(value: ID3D10Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Device1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Device1 {
    type Vtable = ID3D10Device1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c8f_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelshader: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvertexshader: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputlayout: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::windows::core::RawPtr, pstrides: *const u32, poffsets: *const u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: ::windows::core::RawPtr, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicate: ::windows::core::RawPtr, predicatevalue: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const ::windows::core::RawPtr, pdepthstencilview: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstate: ::windows::core::RawPtr, blendfactor: *const f32, samplemask: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilstate: ::windows::core::RawPtr, stencilref: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const ::windows::core::RawPtr, poffsets: *const u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerstate: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, psrcbox: *const D3D10_BOX),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, psrcresource: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertargetview: ::windows::core::RawPtr, colorrgba: *const f32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencilview: ::windows::core::RawPtr, clearflags: u32, depth: f32, stencil: u8),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderresourceview: ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdstresource: ::windows::core::RawPtr, dstsubresource: u32, psrcresource: ::windows::core::RawPtr, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppixelshader: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvertexshader: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinputlayout: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::windows::core::RawPtr, pstrides: *mut u32, poffsets: *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::windows::core::RawPtr, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppredicate: *mut ::windows::core::RawPtr, ppredicatevalue: *mut super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut ::windows::core::RawPtr, ppdepthstencilview: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppblendstate: *mut ::windows::core::RawPtr, blendfactor: *mut f32, psamplemask: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut ::windows::core::RawPtr, pstencilref: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut ::windows::core::RawPtr, poffsets: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: super::super::Foundation::PSTR, pnamelength: *mut u32, szunits: super::super::Foundation::PSTR, punitslength: *mut u32, szdescription: super::super::Foundation::PSTR, pdescriptionlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows::core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32),
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1,
);
#[repr(transparent)]
pub struct ID3D10DeviceChild(::windows::core::IUnknown);
impl ID3D10DeviceChild {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10DeviceChild> for ::windows::core::IUnknown {
    fn from(value: ID3D10DeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10DeviceChild> for ::windows::core::IUnknown {
    fn from(value: &ID3D10DeviceChild) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10DeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10DeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10DeviceChild {
    type Vtable = ID3D10DeviceChildVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c00_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DeviceChildVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10Effect(::windows::core::IUnknown);
impl ID3D10Effect {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPool(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Device>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_DESC> {
        let mut result__: D3D10_EFFECT_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConstantBufferByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVariableByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVariableBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetTechniqueByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTechniqueByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectTechnique> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn Optimize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptimized(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10Effect> for ::windows::core::IUnknown {
    fn from(value: ID3D10Effect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Effect> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Effect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Effect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Effect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Effect {
    type Vtable = ID3D10EffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51b0ca8b_ec0b_4519_870d_8ee1cb5017c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectTechnique>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectBlendVariable(::windows::core::IUnknown);
impl ID3D10EffectBlendVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetBlendState(&self, index: u32) -> ::windows::core::Result<ID3D10BlendState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10BlendState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pblenddesc)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectBlendVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectBlendVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectBlendVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectBlendVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectBlendVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectBlendVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectBlendVariable {
    type Vtable = ID3D10EffectBlendVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fcd2294_df6d_4eae_86b3_0e9160cfb07b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectBlendVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectConstantBuffer(::windows::core::IUnknown);
impl ID3D10EffectConstantBuffer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetConstantBuffer<'a, Param0: ::windows::core::IntoParam<'a, ID3D10Buffer>>(&self, pconstantbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pconstantbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetConstantBuffer(&self) -> ::windows::core::Result<ID3D10Buffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Buffer>(result__)
    }
    pub unsafe fn SetTextureBuffer<'a, Param0: ::windows::core::IntoParam<'a, ID3D10ShaderResourceView>>(&self, ptexturebuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ptexturebuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetTextureBuffer(&self) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderResourceView>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectConstantBuffer> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectConstantBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectConstantBuffer> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectConstantBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectConstantBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectConstantBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectConstantBuffer {
    type Vtable = ID3D10EffectConstantBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56648f4d_cc8b_4444_a5ad_b5a3d76e91b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectConstantBufferVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstantbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptexturebuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilVariable(::windows::core::IUnknown);
impl ID3D10EffectDepthStencilVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetDepthStencilState(&self, index: u32) -> ::windows::core::Result<ID3D10DepthStencilState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_DEPTH_STENCIL_DESC> {
        let mut result__: D3D10_DEPTH_STENCIL_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_DEPTH_STENCIL_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectDepthStencilVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectDepthStencilVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectDepthStencilVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectDepthStencilVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectDepthStencilVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectDepthStencilVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectDepthStencilVariable {
    type Vtable = ID3D10EffectDepthStencilVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf482368_330a_46a5_9a5c_01c71af24c8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilViewVariable(::windows::core::IUnknown);
impl ID3D10EffectDepthStencilViewVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetDepthStencil<'a, Param0: ::windows::core::IntoParam<'a, ID3D10DepthStencilView>>(&self, presource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), presource.into_param().abi()).ok()
    }
    pub unsafe fn GetDepthStencil(&self) -> ::windows::core::Result<ID3D10DepthStencilView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10DepthStencilView>(result__)
    }
    pub unsafe fn SetDepthStencilArray(&self, ppresources: *const ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetDepthStencilArray(&self, ppresources: *mut ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectDepthStencilViewVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectDepthStencilViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectDepthStencilViewVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectDepthStencilViewVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectDepthStencilViewVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectDepthStencilViewVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectDepthStencilViewVariable {
    type Vtable = ID3D10EffectDepthStencilViewVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e02c918_cc79_4985_b622_2d92ad701623);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilViewVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectMatrixVariable(::windows::core::IUnknown);
impl ID3D10EffectMatrixVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetMatrix(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetMatrixTranspose(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectMatrixVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectMatrixVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectMatrixVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectMatrixVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectMatrixVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectMatrixVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectMatrixVariable {
    type Vtable = ID3D10EffectMatrixVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50666c24_b82f_4eed_a172_5b6e7e8522e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectMatrixVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectPass(::windows::core::IUnknown);
impl ID3D10EffectPass {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn Apply(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
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
unsafe impl ::windows::core::Interface for ID3D10EffectPass {
    type Vtable = ID3D10EffectPassVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cfbeb89_1a06_46e0_b282_e3f9bfa36a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPassVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectPool(::windows::core::IUnknown);
impl ID3D10EffectPool {
    pub unsafe fn AsEffect(&self) -> ::core::option::Option<ID3D10Effect> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10EffectPool> for ::windows::core::IUnknown {
    fn from(value: ID3D10EffectPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectPool> for ::windows::core::IUnknown {
    fn from(value: &ID3D10EffectPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10EffectPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10EffectPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectPool {
    type Vtable = ID3D10EffectPoolVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9537ab04_3250_412e_8213_fcd2f8677933);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPoolVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect>);
#[repr(transparent)]
pub struct ID3D10EffectRasterizerVariable(::windows::core::IUnknown);
impl ID3D10EffectRasterizerVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRasterizerState(&self, index: u32) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RasterizerState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_RASTERIZER_DESC> {
        let mut result__: D3D10_RASTERIZER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_RASTERIZER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectRasterizerVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectRasterizerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectRasterizerVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectRasterizerVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectRasterizerVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectRasterizerVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectRasterizerVariable {
    type Vtable = ID3D10EffectRasterizerVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21af9f0e_4d94_4ea9_9785_2cb76b8c0b34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRasterizerVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectRenderTargetViewVariable(::windows::core::IUnknown);
impl ID3D10EffectRenderTargetViewVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetRenderTarget<'a, Param0: ::windows::core::IntoParam<'a, ID3D10RenderTargetView>>(&self, presource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), presource.into_param().abi()).ok()
    }
    pub unsafe fn GetRenderTarget(&self) -> ::windows::core::Result<ID3D10RenderTargetView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10RenderTargetView>(result__)
    }
    pub unsafe fn SetRenderTargetArray(&self, ppresources: *const ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetRenderTargetArray(&self, ppresources: *mut ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectRenderTargetViewVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectRenderTargetViewVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectRenderTargetViewVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectRenderTargetViewVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectRenderTargetViewVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectRenderTargetViewVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectRenderTargetViewVariable {
    type Vtable = ID3D10EffectRenderTargetViewVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28ca0cc3_c2c9_40bb_b57f_67b737122b17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRenderTargetViewVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectSamplerVariable(::windows::core::IUnknown);
impl ID3D10EffectSamplerVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetSampler(&self, index: u32) -> ::windows::core::Result<ID3D10SamplerState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10SamplerState>(result__)
    }
    pub unsafe fn GetBackingStore(&self, index: u32) -> ::windows::core::Result<D3D10_SAMPLER_DESC> {
        let mut result__: D3D10_SAMPLER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SAMPLER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectSamplerVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectSamplerVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectSamplerVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectSamplerVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectSamplerVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectSamplerVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectSamplerVariable {
    type Vtable = ID3D10EffectSamplerVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6530d5c7_07e9_4271_a418_e7ce4bd1e480);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectSamplerVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectScalarVariable(::windows::core::IUnknown);
impl ID3D10EffectScalarVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetFloat(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetFloat(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    pub unsafe fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SetInt(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetInt(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBool<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBool(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolArray(&self, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectScalarVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectScalarVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectScalarVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectScalarVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectScalarVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectScalarVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectScalarVariable {
    type Vtable = ID3D10EffectScalarVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e48f7b_d2c8_49e8_a86c_022dee53431f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectScalarVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectShaderResourceVariable(::windows::core::IUnknown);
impl ID3D10EffectShaderResourceVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn SetResource<'a, Param0: ::windows::core::IntoParam<'a, ID3D10ShaderResourceView>>(&self, presource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), presource.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10ShaderResourceView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10ShaderResourceView>(result__)
    }
    pub unsafe fn SetResourceArray(&self, ppresources: *const ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetResourceArray(&self, ppresources: *mut ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectShaderResourceVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectShaderResourceVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectShaderResourceVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectShaderResourceVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectShaderResourceVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectShaderResourceVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectShaderResourceVariable {
    type Vtable = ID3D10EffectShaderResourceVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0a7157b_d872_4b1d_8073_efc2acd4b1fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderResourceVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr, offset: u32, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectShaderVariable(::windows::core::IUnknown);
impl ID3D10EffectShaderVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetShaderDesc(&self, shaderindex: u32) -> ::windows::core::Result<D3D10_EFFECT_SHADER_DESC> {
        let mut result__: D3D10_EFFECT_SHADER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_SHADER_DESC>(result__)
    }
    pub unsafe fn GetVertexShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10VertexShader>(result__)
    }
    pub unsafe fn GetGeometryShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10GeometryShader>(result__)
    }
    pub unsafe fn GetPixelShader(&self, shaderindex: u32) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10PixelShader>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(element), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(shaderindex), ::core::mem::transmute(element), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10EffectShaderVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectShaderVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectShaderVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectShaderVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectShaderVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectShaderVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectShaderVariable {
    type Vtable = ID3D10EffectShaderVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80849279_c799_4797_8c33_0407a07d9e06);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectStringVariable(::windows::core::IUnknown);
impl ID3D10EffectStringVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self) -> ::windows::core::Result<super::super::Foundation::PSTR> {
        let mut result__: super::super::Foundation::PSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringArray(&self, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppstrings), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectStringVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectStringVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectStringVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectStringVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectStringVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectStringVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectStringVariable {
    type Vtable = ID3D10EffectStringVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71417501_8df9_4e0a_a78a_255f9756baff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectStringVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstring: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrings: *mut super::super::Foundation::PSTR, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectTechnique(::windows::core::IUnknown);
impl ID3D10EffectTechnique {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetPassByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectPass> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn ComputeStateBlockMask(&self) -> ::windows::core::Result<D3D10_STATE_BLOCK_MASK> {
        let mut result__: D3D10_STATE_BLOCK_MASK = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_STATE_BLOCK_MASK>(result__)
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
unsafe impl ::windows::core::Interface for ID3D10EffectTechnique {
    type Vtable = ID3D10EffectTechniqueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb122ce8_d1c9_4292_b237_24ed3de8b175);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectTechniqueVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectPass>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectType(::windows::core::IUnknown);
impl ID3D10EffectType {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberTypeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberTypeBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberName(&self, index: u32) -> super::super::Foundation::PSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberSemantic(&self, index: u32) -> super::super::Foundation::PSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
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
unsafe impl ::windows::core::Interface for ID3D10EffectType {
    type Vtable = ID3D10EffectTypeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9e1ddc_cd9d_4772_a837_00180b9b88fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectTypeVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10EffectVariable(::windows::core::IUnknown);
impl ID3D10EffectVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
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
unsafe impl ::windows::core::Interface for ID3D10EffectVariable {
    type Vtable = ID3D10EffectVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae897105_00e6_45bf_bb8e_281dd6db8e1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10EffectVectorVariable(::windows::core::IUnknown);
impl ID3D10EffectVectorVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_EFFECT_VARIABLE_DESC> {
        let mut result__: D3D10_EFFECT_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_EFFECT_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnnotationByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberBySemantic<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, semantic: Param0) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), semantic.into_param().abi()))
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(bytecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetIntVector(&self, pdata: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn GetFloatVector(&self, pdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(offset), ::core::mem::transmute(count)).ok()
    }
}
impl ::core::convert::From<ID3D10EffectVectorVariable> for ID3D10EffectVariable {
    fn from(value: ID3D10EffectVectorVariable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10EffectVectorVariable> for ID3D10EffectVariable {
    fn from(value: &ID3D10EffectVectorVariable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for ID3D10EffectVectorVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10EffectVariable> for &ID3D10EffectVectorVariable {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10EffectVariable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10EffectVectorVariable {
    type Vtable = ID3D10EffectVectorVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b98c44_1f82_4c67_bcd0_72cf8f217e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVectorVariableVtbl(
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10EffectVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable>,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10GeometryShader(::windows::core::IUnknown);
impl ID3D10GeometryShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10GeometryShader> for ID3D10DeviceChild {
    fn from(value: ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10GeometryShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10GeometryShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10GeometryShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10GeometryShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10GeometryShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10GeometryShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10GeometryShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10GeometryShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10GeometryShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10GeometryShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10GeometryShader {
    type Vtable = ID3D10GeometryShaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6316be88_54cd_4040_ab44_20461bc81f68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10GeometryShaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10InfoQueue(::windows::core::IUnknown);
impl ID3D10InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(messagecountlimit)).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(messageindex), ::core::mem::transmute(pmessage), ::core::mem::transmute(pmessagebytelength)).ok()
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    pub unsafe fn ClearStorageFilter(&self) {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter), ::core::mem::transmute(pfilterbytelength)).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilter)).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(severity), ::core::mem::transmute(id), pdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddApplicationMessage<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(severity), pdescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, category: D3D10_MESSAGE_CATEGORY, benable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), benable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, severity: D3D10_MESSAGE_SEVERITY, benable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(severity), benable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, id: D3D10_MESSAGE_ID, benable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), benable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(category)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(severity)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmute: Param0) {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bmute.into_param().abi())
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10InfoQueue> for ::windows::core::IUnknown {
    fn from(value: ID3D10InfoQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InfoQueue> for ::windows::core::IUnknown {
    fn from(value: &ID3D10InfoQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10InfoQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10InfoQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10InfoQueue {
    type Vtable = ID3D10InfoQueueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b940b17_2642_4d1f_ab1f_b99bad0c395f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InfoQueueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10InputLayout(::windows::core::IUnknown);
impl ID3D10InputLayout {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10InputLayout> for ID3D10DeviceChild {
    fn from(value: ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InputLayout> for ID3D10DeviceChild {
    fn from(value: &ID3D10InputLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10InputLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10InputLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10InputLayout> for ::windows::core::IUnknown {
    fn from(value: ID3D10InputLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10InputLayout> for ::windows::core::IUnknown {
    fn from(value: &ID3D10InputLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10InputLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10InputLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10InputLayout {
    type Vtable = ID3D10InputLayoutVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0b_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InputLayoutVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10Multithread(::windows::core::IUnknown);
impl ID3D10Multithread {
    pub unsafe fn Enter(&self) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn Leave(&self) {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMultithreadProtected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmtprotect: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bmtprotect.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10Multithread> for ::windows::core::IUnknown {
    fn from(value: ID3D10Multithread) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Multithread> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Multithread) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Multithread {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Multithread {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Multithread {
    type Vtable = ID3D10MultithreadVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10MultithreadVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10PixelShader(::windows::core::IUnknown);
impl ID3D10PixelShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10PixelShader> for ID3D10DeviceChild {
    fn from(value: ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10PixelShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10PixelShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10PixelShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10PixelShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10PixelShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10PixelShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10PixelShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10PixelShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10PixelShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10PixelShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10PixelShader {
    type Vtable = ID3D10PixelShaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4968b601_9d00_4cde_8346_8e7f675819b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10PixelShaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10Predicate(::windows::core::IUnknown);
impl ID3D10Predicate {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(datasize), ::core::mem::transmute(getdataflags)).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10Query {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10Query {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Query> for ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Query> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Query> for &ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Query> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10Asynchronous {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10Asynchronous {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for &ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Predicate> for ID3D10DeviceChild {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ID3D10DeviceChild {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Predicate> for ::windows::core::IUnknown {
    fn from(value: ID3D10Predicate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Predicate> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Predicate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Predicate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Predicate {
    type Vtable = ID3D10PredicateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c10_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10PredicateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC),
);
#[repr(transparent)]
pub struct ID3D10Query(::windows::core::IUnknown);
impl ID3D10Query {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn Begin(&self) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self))
    }
    pub unsafe fn GetData(&self, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(datasize), ::core::mem::transmute(getdataflags)).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Query> for ID3D10Asynchronous {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ID3D10Asynchronous {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Asynchronous> for &ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Asynchronous> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Query> for ID3D10DeviceChild {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ID3D10DeviceChild {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Query> for ::windows::core::IUnknown {
    fn from(value: ID3D10Query) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Query> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Query) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Query {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Query {
    type Vtable = ID3D10QueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0e_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10QueryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC),
);
#[repr(transparent)]
pub struct ID3D10RasterizerState(::windows::core::IUnknown);
impl ID3D10RasterizerState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10RasterizerState> for ID3D10DeviceChild {
    fn from(value: ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RasterizerState> for ID3D10DeviceChild {
    fn from(value: &ID3D10RasterizerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10RasterizerState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10RasterizerState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10RasterizerState> for ::windows::core::IUnknown {
    fn from(value: ID3D10RasterizerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RasterizerState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10RasterizerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10RasterizerState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10RasterizerState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10RasterizerState {
    type Vtable = ID3D10RasterizerStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2a07292_89af_4345_be2e_c53d9fbb6e9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RasterizerStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10RenderTargetView(::windows::core::IUnknown);
impl ID3D10RenderTargetView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresource))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ID3D10View {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ID3D10View {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for &ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ID3D10DeviceChild {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ID3D10DeviceChild {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10RenderTargetView> for ::windows::core::IUnknown {
    fn from(value: ID3D10RenderTargetView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10RenderTargetView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10RenderTargetView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10RenderTargetView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10RenderTargetView {
    type Vtable = ID3D10RenderTargetViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c08_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RenderTargetViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Resource(::windows::core::IUnknown);
impl ID3D10Resource {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority))
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10Resource> for ID3D10DeviceChild {
    fn from(value: ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Resource> for ID3D10DeviceChild {
    fn from(value: &ID3D10Resource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Resource {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Resource {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Resource> for ::windows::core::IUnknown {
    fn from(value: ID3D10Resource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Resource> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Resource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Resource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Resource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Resource {
    type Vtable = ID3D10ResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c01_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
);
#[repr(transparent)]
pub struct ID3D10SamplerState(::windows::core::IUnknown);
impl ID3D10SamplerState {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10SamplerState> for ID3D10DeviceChild {
    fn from(value: ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SamplerState> for ID3D10DeviceChild {
    fn from(value: &ID3D10SamplerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10SamplerState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10SamplerState {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10SamplerState> for ::windows::core::IUnknown {
    fn from(value: ID3D10SamplerState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SamplerState> for ::windows::core::IUnknown {
    fn from(value: &ID3D10SamplerState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10SamplerState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10SamplerState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10SamplerState {
    type Vtable = ID3D10SamplerStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0c_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SamplerStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC),
);
#[repr(transparent)]
pub struct ID3D10ShaderReflection(::windows::core::IUnknown);
impl ID3D10ShaderReflection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC> {
        let mut result__: D3D10_SHADER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConstantBufferByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC> {
        let mut result__: D3D10_SHADER_INPUT_BIND_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourceindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
}
impl ::core::convert::From<ID3D10ShaderReflection> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderReflection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderReflection> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderReflection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10ShaderReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10ShaderReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10ShaderReflection {
    type Vtable = ID3D10ShaderReflectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd40e20b6_f8f7_42ad_ab20_4baf8f15dfaa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
);
#[repr(transparent)]
pub struct ID3D10ShaderReflection1(::windows::core::IUnknown);
impl ID3D10ShaderReflection1 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_DESC> {
        let mut result__: D3D10_SHADER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_DESC>(result__)
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConstantBufferByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC> {
        let mut result__: D3D10_SHADER_INPUT_BIND_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourceindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32) -> ::windows::core::Result<D3D10_SIGNATURE_PARAMETER_DESC> {
        let mut result__: D3D10_SIGNATURE_PARAMETER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterindex), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SIGNATURE_PARAMETER_DESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVariableByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetResourceBindingDescByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::windows::core::Result<D3D10_SHADER_INPUT_BIND_DESC> {
        let mut result__: D3D10_SHADER_INPUT_BIND_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_INPUT_BIND_DESC>(result__)
    }
    pub unsafe fn GetMovInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> ::windows::core::Result<super::Direct3D::D3D_PRIMITIVE> {
        let mut result__: super::Direct3D::D3D_PRIMITIVE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Direct3D::D3D_PRIMITIVE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLevel9Shader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSampleFrequencyShader(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ID3D10ShaderReflection1> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderReflection1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderReflection1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderReflection1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10ShaderReflection1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10ShaderReflection1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10ShaderReflection1 {
    type Vtable = ID3D10ShaderReflection1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3457783_a846_47ce_9520_cea6f66e7447);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionConstantBuffer(::windows::core::IUnknown);
impl ID3D10ShaderReflectionConstantBuffer {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_BUFFER_DESC> {
        let mut result__: D3D10_SHADER_BUFFER_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_BUFFER_DESC>(result__)
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVariableByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), name.into_param().abi()))
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
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionConstantBuffer {
    type Vtable = ID3D10ShaderReflectionConstantBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66c66a94_dddd_4b62_a66a_f0da33c2b4d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionConstantBufferVtbl(
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionType(::windows::core::IUnknown);
impl ID3D10ShaderReflectionType {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc)).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberTypeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(&self, name: Param0) -> ::core::option::Option<ID3D10ShaderReflectionType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).2)(::core::mem::transmute_copy(self), name.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> super::super::Foundation::PSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
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
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionType {
    type Vtable = ID3D10ShaderReflectionTypeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc530ad7d_9b16_4395_a979_ba2ecff83add);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionTypeVtbl(
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType>,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PSTR) -> ::core::option::Option<ID3D10ShaderReflectionType>,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> super::super::Foundation::PSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionVariable(::windows::core::IUnknown);
impl ID3D10ShaderReflectionVariable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self) -> ::windows::core::Result<D3D10_SHADER_VARIABLE_DESC> {
        let mut result__: D3D10_SHADER_VARIABLE_DESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).0)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_SHADER_VARIABLE_DESC>(result__)
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10ShaderReflectionType> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).1)(::core::mem::transmute_copy(self)))
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
unsafe impl ::windows::core::Interface for ID3D10ShaderReflectionVariable {
    type Vtable = ID3D10ShaderReflectionVariableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf63c95_2650_405d_99c1_3636bd1da0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionVariableVtbl(#[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows::core::HRESULT, #[cfg(not(feature = "Win32_Foundation"))] usize, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType>);
#[repr(transparent)]
pub struct ID3D10ShaderResourceView(::windows::core::IUnknown);
impl ID3D10ShaderResourceView {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresource))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ID3D10View {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ID3D10View {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for &ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ID3D10DeviceChild {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ID3D10DeviceChild {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderResourceView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderResourceView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10ShaderResourceView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10ShaderResourceView {
    type Vtable = ID3D10ShaderResourceViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c07_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr),
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[repr(transparent)]
pub struct ID3D10ShaderResourceView1(::windows::core::IUnknown);
impl ID3D10ShaderResourceView1 {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresource))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10ShaderResourceView {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10ShaderResourceView {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10ShaderResourceView> for ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10ShaderResourceView> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10ShaderResourceView> for &ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10ShaderResourceView> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10View {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10View {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10View> for &ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10View> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ID3D10DeviceChild {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ID3D10DeviceChild {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10ShaderResourceView1> for ::windows::core::IUnknown {
    fn from(value: ID3D10ShaderResourceView1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10ShaderResourceView1> for ::windows::core::IUnknown {
    fn from(value: &ID3D10ShaderResourceView1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10ShaderResourceView1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10ShaderResourceView1 {
    type Vtable = ID3D10ShaderResourceView1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c87_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr),
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[repr(transparent)]
pub struct ID3D10StateBlock(::windows::core::IUnknown);
impl ID3D10StateBlock {
    pub unsafe fn Capture(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReleaseAllDeviceObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ID3D10Device>(result__)
    }
}
impl ::core::convert::From<ID3D10StateBlock> for ::windows::core::IUnknown {
    fn from(value: ID3D10StateBlock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10StateBlock> for ::windows::core::IUnknown {
    fn from(value: &ID3D10StateBlock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10StateBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10StateBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10StateBlock {
    type Vtable = ID3D10StateBlockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0803425a_57f5_4dd6_9465_a87570834a08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10StateBlockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10SwitchToRef(::windows::core::IUnknown);
impl ID3D10SwitchToRef {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseRef<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, useref: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), useref.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseRef(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<ID3D10SwitchToRef> for ::windows::core::IUnknown {
    fn from(value: ID3D10SwitchToRef) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10SwitchToRef> for ::windows::core::IUnknown {
    fn from(value: &ID3D10SwitchToRef) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10SwitchToRef {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10SwitchToRef {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10SwitchToRef {
    type Vtable = ID3D10SwitchToRefVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4e02_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SwitchToRefVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Texture1D(::windows::core::IUnknown);
impl ID3D10Texture1D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority))
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource), ::core::mem::transmute(maptype), ::core::mem::transmute(mapflags), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ID3D10Resource {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ID3D10Resource {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for &ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture1D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture1D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture1D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture1D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Texture1D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Texture1D {
    type Vtable = ID3D10Texture1DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c03_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture1DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Texture2D(::windows::core::IUnknown);
impl ID3D10Texture2D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority))
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE2D> {
        let mut result__: D3D10_MAPPED_TEXTURE2D = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource), ::core::mem::transmute(maptype), ::core::mem::transmute(mapflags), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_MAPPED_TEXTURE2D>(result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ID3D10Resource {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ID3D10Resource {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for &ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture2D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture2D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Texture2D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Texture2D {
    type Vtable = ID3D10Texture2DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c04_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture2DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[repr(transparent)]
pub struct ID3D10Texture3D(::windows::core::IUnknown);
impl ID3D10Texture3D {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtype))
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(evictionpriority))
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows::core::Result<D3D10_MAPPED_TEXTURE3D> {
        let mut result__: D3D10_MAPPED_TEXTURE3D = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource), ::core::mem::transmute(maptype), ::core::mem::transmute(mapflags), ::core::mem::transmute(&mut result__)).from_abi::<D3D10_MAPPED_TEXTURE3D>(result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(subresource))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC) {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdesc))
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ID3D10Resource {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ID3D10Resource {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10Resource> for &ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10Resource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ID3D10DeviceChild {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ID3D10DeviceChild {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10Texture3D> for ::windows::core::IUnknown {
    fn from(value: ID3D10Texture3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10Texture3D> for ::windows::core::IUnknown {
    fn from(value: &ID3D10Texture3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10Texture3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10Texture3D {
    type Vtable = ID3D10Texture3DVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c05_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture3DVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, evictionpriority: u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subresource: u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[repr(transparent)]
pub struct ID3D10VertexShader(::windows::core::IUnknown);
impl ID3D10VertexShader {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ID3D10VertexShader> for ID3D10DeviceChild {
    fn from(value: ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10VertexShader> for ID3D10DeviceChild {
    fn from(value: &ID3D10VertexShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10VertexShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10VertexShader {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10VertexShader> for ::windows::core::IUnknown {
    fn from(value: ID3D10VertexShader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10VertexShader> for ::windows::core::IUnknown {
    fn from(value: &ID3D10VertexShader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10VertexShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10VertexShader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10VertexShader {
    type Vtable = ID3D10VertexShaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e4c0a_342c_4106_a19f_4f2704f689f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10VertexShaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ID3D10View(::windows::core::IUnknown);
impl ID3D10View {
    pub unsafe fn GetDevice(&self, ppdevice: *mut ::core::option::Option<ID3D10Device>) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdevice))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, pdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), pdata.into_param().abi()).ok()
    }
    pub unsafe fn GetResource(&self, ppresource: *mut ::core::option::Option<ID3D10Resource>) {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresource))
    }
}
impl ::core::convert::From<ID3D10View> for ID3D10DeviceChild {
    fn from(value: ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10View> for ID3D10DeviceChild {
    fn from(value: &ID3D10View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for ID3D10View {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D10DeviceChild> for &ID3D10View {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D10DeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D10View> for ::windows::core::IUnknown {
    fn from(value: ID3D10View) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D10View> for ::windows::core::IUnknown {
    fn from(value: &ID3D10View) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D10View {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ID3D10View {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ID3D10View {
    type Vtable = ID3D10ViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc902b03f_60a7_49ba_9936_2a3ab37a7e33);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresource: *mut ::windows::core::RawPtr),
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub type PFN_D3D10_CREATE_DEVICE1 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::Dxgi::DXGI_SWAP_CHAIN_DESC, param7: *mut ::core::option::Option<super::Dxgi::IDXGISwapChain>, param8: *mut ::core::option::Option<ID3D10Device1>) -> ::windows::core::HRESULT>;
pub const _FACD3D10: u32 = 2169u32;
