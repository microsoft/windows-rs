#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcAssembler_Impl: Sized {
    fn AssembleToContainer(&self, pshader: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcOperationResult>;
}
impl ::windows::core::RuntimeName for IDxcAssembler {}
impl IDxcAssembler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: isize>() -> IDxcAssembler_Vtbl {
        unsafe extern "system" fn AssembleToContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssembleToContainer(::windows::core::from_raw_borrowed(&pshader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AssembleToContainer: AssembleToContainer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcAssembler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcBlob_Impl: Sized {
    fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl ::windows::core::RuntimeName for IDxcBlob {}
impl IDxcBlob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>() -> IDxcBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferSize()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlob as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobEncoding_Impl: Sized + IDxcBlob_Impl {
    fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDxcBlobEncoding {}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobEncoding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>() -> IDxcBlobEncoding_Vtbl {
        unsafe extern "system" fn GetEncoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEncoding(::core::mem::transmute_copy(&pknown), ::core::mem::transmute_copy(&pcodepage)).into()
        }
        Self { base__: IDxcBlob_Vtbl::new::<Identity, Impl, OFFSET>(), GetEncoding: GetEncoding::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as ::windows::core::ComInterface>::IID || iid == &<IDxcBlob as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf16_Impl: Sized + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> ::windows::core::PCWSTR;
    fn GetStringLength(&self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDxcBlobUtf16 {}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf16_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>() -> IDxcBlobUtf16_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringLength()
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf16 as ::windows::core::ComInterface>::IID || iid == &<IDxcBlob as ::windows::core::ComInterface>::IID || iid == &<IDxcBlobEncoding as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf8_Impl: Sized + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> ::windows::core::PCSTR;
    fn GetStringLength(&self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDxcBlobUtf8 {}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>() -> IDxcBlobUtf8_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::PCSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringLength()
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as ::windows::core::ComInterface>::IID || iid == &<IDxcBlob as ::windows::core::ComInterface>::IID || iid == &<IDxcBlobEncoding as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcCompiler_Impl: Sized {
    fn Compile(&self, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows::core::PCWSTR, pentrypoint: &::windows::core::PCWSTR, ptargetprofile: &::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Preprocess(&self, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Disassemble(&self, psource: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
impl ::windows::core::RuntimeName for IDxcCompiler {}
impl IDxcCompiler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>() -> IDxcCompiler_Vtbl {
        unsafe extern "system" fn Compile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compile(::windows::core::from_raw_borrowed(&psource), ::core::mem::transmute(&psourcename), ::core::mem::transmute(&pentrypoint), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::windows::core::from_raw_borrowed(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preprocess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Preprocess(::windows::core::from_raw_borrowed(&psource), ::core::mem::transmute(&psourcename), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::windows::core::from_raw_borrowed(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Disassemble(::windows::core::from_raw_borrowed(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisassembly, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcCompiler2_Impl: Sized + IDxcCompiler_Impl {
    fn CompileWithDebug(&self, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows::core::PCWSTR, pentrypoint: &::windows::core::PCWSTR, ptargetprofile: &::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut ::windows::core::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcCompiler2 {}
impl IDxcCompiler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: isize>() -> IDxcCompiler2_Vtbl {
        unsafe extern "system" fn CompileWithDebug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void, ppdebugblobname: *mut ::windows::core::PWSTR, ppdebugblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompileWithDebug(
                ::windows::core::from_raw_borrowed(&psource),
                ::core::mem::transmute(&psourcename),
                ::core::mem::transmute(&pentrypoint),
                ::core::mem::transmute(&ptargetprofile),
                ::core::mem::transmute_copy(&parguments),
                ::core::mem::transmute_copy(&argcount),
                ::core::mem::transmute_copy(&pdefines),
                ::core::mem::transmute_copy(&definecount),
                ::windows::core::from_raw_borrowed(&pincludehandler),
                ::core::mem::transmute_copy(&ppresult),
                ::core::mem::transmute_copy(&ppdebugblobname),
                ::core::mem::transmute_copy(&ppdebugblob),
            )
            .into()
        }
        Self { base__: IDxcCompiler_Vtbl::new::<Identity, Impl, OFFSET>(), CompileWithDebug: CompileWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler2 as ::windows::core::ComInterface>::IID || iid == &<IDxcCompiler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcCompiler3_Impl: Sized {
    fn Compile(&self, psource: *const DxcBuffer, parguments: *const ::windows::core::PCWSTR, argcount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcCompiler3 {}
impl IDxcCompiler3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>() -> IDxcCompiler3_Vtbl {
        unsafe extern "system" fn Compile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows::core::PCWSTR, argcount: u32, pincludehandler: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compile(::core::mem::transmute_copy(&psource), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::windows::core::from_raw_borrowed(&pincludehandler), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disassemble(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcCompilerArgs_Impl: Sized {
    fn GetArguments(&self) -> *mut ::windows::core::PCWSTR;
    fn GetCount(&self) -> u32;
    fn AddArguments(&self, parguments: *const ::windows::core::PCWSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddArgumentsUTF8(&self, parguments: *const ::windows::core::PCSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddDefines(&self, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcCompilerArgs {}
impl IDxcCompilerArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>() -> IDxcCompilerArgs_Vtbl {
        unsafe extern "system" fn GetArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::windows::core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetArguments()
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount()
        }
        unsafe extern "system" fn AddArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PCWSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddArguments(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PCSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddArgumentsUTF8(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddDefines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDefines(::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetArguments: GetArguments::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddArguments: AddArguments::<Identity, Impl, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, Impl, OFFSET>,
            AddDefines: AddDefines::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcContainerBuilder_Impl: Sized {
    fn Load(&self, pdxilcontainerheader: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<()>;
    fn AddPart(&self, fourcc: u32, psource: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<()>;
    fn RemovePart(&self, fourcc: u32) -> ::windows::core::Result<()>;
    fn SerializeContainer(&self) -> ::windows::core::Result<IDxcOperationResult>;
}
impl ::windows::core::RuntimeName for IDxcContainerBuilder {}
impl IDxcContainerBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>() -> IDxcContainerBuilder_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdxilcontainerheader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::windows::core::from_raw_borrowed(&pdxilcontainerheader)).into()
        }
        unsafe extern "system" fn AddPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32, psource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPart(::core::mem::transmute_copy(&fourcc), ::windows::core::from_raw_borrowed(&psource)).into()
        }
        unsafe extern "system" fn RemovePart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePart(::core::mem::transmute_copy(&fourcc)).into()
        }
        unsafe extern "system" fn SerializeContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SerializeContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            AddPart: AddPart::<Identity, Impl, OFFSET>,
            RemovePart: RemovePart::<Identity, Impl, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcContainerReflection_Impl: Sized {
    fn Load(&self, pcontainer: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<()>;
    fn GetPartCount(&self) -> ::windows::core::Result<u32>;
    fn GetPartKind(&self, idx: u32) -> ::windows::core::Result<u32>;
    fn GetPartContent(&self, idx: u32) -> ::windows::core::Result<IDxcBlob>;
    fn FindFirstPartKind(&self, kind: u32) -> ::windows::core::Result<u32>;
    fn GetPartReflection(&self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcContainerReflection {}
impl IDxcContainerReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>() -> IDxcContainerReflection_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontainer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::windows::core::from_raw_borrowed(&pcontainer)).into()
        }
        unsafe extern "system" fn GetPartCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartKind(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartContent(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindFirstPartKind(::core::mem::transmute_copy(&kind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartReflection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPartReflection(::core::mem::transmute_copy(&idx), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            GetPartCount: GetPartCount::<Identity, Impl, OFFSET>,
            GetPartKind: GetPartKind::<Identity, Impl, OFFSET>,
            GetPartContent: GetPartContent::<Identity, Impl, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, Impl, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerReflection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcExtraOutputs_Impl: Sized {
    fn GetOutputCount(&self) -> u32;
    fn GetOutput(&self, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcExtraOutputs {}
impl IDxcExtraOutputs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>() -> IDxcExtraOutputs_Vtbl {
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputCount()
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutput(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputtype), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcIncludeHandler_Impl: Sized {
    fn LoadSource(&self, pfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<IDxcBlob>;
}
impl ::windows::core::RuntimeName for IDxcIncludeHandler {}
impl IDxcIncludeHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>() -> IDxcIncludeHandler_Vtbl {
        unsafe extern "system" fn LoadSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, ppincludesource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadSource(::core::mem::transmute(&pfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppincludesource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadSource: LoadSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcLibrary_Impl: Sized {
    fn SetMalloc(&self, pmalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>) -> ::windows::core::Result<()>;
    fn CreateBlobFromBlob(&self, pblob: ::core::option::Option<&IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&self, pfilename: &::windows::core::PCWSTR, codepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&self, ptext: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDxcLibrary {}
#[cfg(feature = "Win32_System_Com")]
impl IDxcLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>() -> IDxcLibrary_Vtbl {
        unsafe extern "system" fn SetMalloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmalloc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMalloc(::windows::core::from_raw_borrowed(&pmalloc)).into()
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobFromBlob(::windows::core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobFromFile(::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobWithEncodingFromPinned(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobWithEncodingOnHeapCopy(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobWithEncodingOnMalloc(::core::mem::transmute_copy(&ptext), ::windows::core::from_raw_borrowed(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStreamFromBlobReadOnly(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlobAsUtf8(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlobAsUtf16(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMalloc: SetMalloc::<Identity, Impl, OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Identity, Impl, OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Identity, Impl, OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcLibrary as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcLinker_Impl: Sized {
    fn RegisterLibrary(&self, plibname: &::windows::core::PCWSTR, plib: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<()>;
    fn Link(&self, pentryname: &::windows::core::PCWSTR, ptargetprofile: &::windows::core::PCWSTR, plibnames: *const ::windows::core::PCWSTR, libcount: u32, parguments: *const ::windows::core::PCWSTR, argcount: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
impl ::windows::core::RuntimeName for IDxcLinker {}
impl IDxcLinker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>() -> IDxcLinker_Vtbl {
        unsafe extern "system" fn RegisterLibrary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibname: ::windows::core::PCWSTR, plib: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterLibrary(::core::mem::transmute(&plibname), ::windows::core::from_raw_borrowed(&plib)).into()
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentryname: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, plibnames: *const ::windows::core::PCWSTR, libcount: u32, parguments: *const ::windows::core::PCWSTR, argcount: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link(::core::mem::transmute(&pentryname), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&plibnames), ::core::mem::transmute_copy(&libcount), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterLibrary: RegisterLibrary::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcLinker as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcOperationResult_Impl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetResult(&self) -> ::windows::core::Result<IDxcBlob>;
    fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding>;
}
impl ::windows::core::RuntimeName for IDxcOperationResult {}
impl IDxcOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>() -> IDxcOperationResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperrors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOperationResult as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcOptimizer_Impl: Sized {
    fn GetAvailablePassCount(&self) -> ::windows::core::Result<u32>;
    fn GetAvailablePass(&self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&self, pblob: ::core::option::Option<&IDxcBlob>, ppoptions: *const ::windows::core::PCWSTR, optioncount: u32, poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcOptimizer {}
impl IDxcOptimizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>() -> IDxcOptimizer_Vtbl {
        unsafe extern "system" fn GetAvailablePassCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailablePassCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailablePass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailablePass(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunOptimizer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppoptions: *const ::windows::core::PCWSTR, optioncount: u32, poutputmodule: *mut *mut ::core::ffi::c_void, ppoutputtext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunOptimizer(::windows::core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&ppoptions), ::core::mem::transmute_copy(&optioncount), ::core::mem::transmute_copy(&poutputmodule), ::core::mem::transmute_copy(&ppoutputtext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Identity, Impl, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, Impl, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcOptimizerPass_Impl: Sized {
    fn GetOptionName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetOptionArgCount(&self) -> ::windows::core::Result<u32>;
    fn GetOptionArgName(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetOptionArgDescription(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IDxcOptimizerPass {}
impl IDxcOptimizerPass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>() -> IDxcOptimizerPass_Vtbl {
        unsafe extern "system" fn GetOptionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionArgName(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOptionArgDescription(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionName: GetOptionName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, Impl, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, Impl, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcPdbUtils_Impl: Sized {
    fn Load(&self, ppdbordxil: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<()>;
    fn GetSourceCount(&self) -> ::windows::core::Result<u32>;
    fn GetSource(&self, uindex: u32) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn GetSourceName(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetFlagCount(&self) -> ::windows::core::Result<u32>;
    fn GetFlag(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetArgCount(&self) -> ::windows::core::Result<u32>;
    fn GetArg(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetArgPairCount(&self) -> ::windows::core::Result<u32>;
    fn GetArgPair(&self, uindex: u32, pname: *mut ::windows::core::BSTR, pvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetDefineCount(&self) -> ::windows::core::Result<u32>;
    fn GetDefine(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetTargetProfile(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetEntryPoint(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetMainFileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetHash(&self) -> ::windows::core::Result<IDxcBlob>;
    fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL;
    fn GetFullPDB(&self) -> ::windows::core::Result<IDxcBlob>;
    fn GetVersionInfo(&self) -> ::windows::core::Result<IDxcVersionInfo>;
    fn SetCompiler(&self, pcompiler: ::core::option::Option<&IDxcCompiler3>) -> ::windows::core::Result<()>;
    fn CompileForFullPDB(&self) -> ::windows::core::Result<IDxcResult>;
    fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()>;
    fn OverrideRootSignature(&self, prootsignature: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDxcPdbUtils {}
#[cfg(feature = "Win32_Foundation")]
impl IDxcPdbUtils_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>() -> IDxcPdbUtils_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbordxil: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::windows::core::from_raw_borrowed(&ppdbordxil)).into()
        }
        unsafe extern "system" fn GetSourceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSource(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceName(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlagCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlagCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlag(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetArg(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPairCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetArgPairCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPair<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetArgPair(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetDefineCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefineCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefine(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetProfile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMainFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHash() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullPDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsFullPDB()
        }
        unsafe extern "system" fn GetFullPDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfullpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfullpdb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppversioninfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppversioninfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompiler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompiler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompiler(::windows::core::from_raw_borrowed(&pcompiler)).into()
        }
        unsafe extern "system" fn CompileForFullPDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompileForFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideArgs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OverrideArgs(::core::mem::transmute_copy(&pargpairs), ::core::mem::transmute_copy(&unumargpairs)).into()
        }
        unsafe extern "system" fn OverrideRootSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OverrideRootSignature(::core::mem::transmute(&prootsignature)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetSourceName: GetSourceName::<Identity, Impl, OFFSET>,
            GetFlagCount: GetFlagCount::<Identity, Impl, OFFSET>,
            GetFlag: GetFlag::<Identity, Impl, OFFSET>,
            GetArgCount: GetArgCount::<Identity, Impl, OFFSET>,
            GetArg: GetArg::<Identity, Impl, OFFSET>,
            GetArgPairCount: GetArgPairCount::<Identity, Impl, OFFSET>,
            GetArgPair: GetArgPair::<Identity, Impl, OFFSET>,
            GetDefineCount: GetDefineCount::<Identity, Impl, OFFSET>,
            GetDefine: GetDefine::<Identity, Impl, OFFSET>,
            GetTargetProfile: GetTargetProfile::<Identity, Impl, OFFSET>,
            GetEntryPoint: GetEntryPoint::<Identity, Impl, OFFSET>,
            GetMainFileName: GetMainFileName::<Identity, Impl, OFFSET>,
            GetHash: GetHash::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            IsFullPDB: IsFullPDB::<Identity, Impl, OFFSET>,
            GetFullPDB: GetFullPDB::<Identity, Impl, OFFSET>,
            GetVersionInfo: GetVersionInfo::<Identity, Impl, OFFSET>,
            SetCompiler: SetCompiler::<Identity, Impl, OFFSET>,
            CompileForFullPDB: CompileForFullPDB::<Identity, Impl, OFFSET>,
            OverrideArgs: OverrideArgs::<Identity, Impl, OFFSET>,
            OverrideRootSignature: OverrideRootSignature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcPdbUtils as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcResult_Impl: Sized + IDxcOperationResult_Impl {
    fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL;
    fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
    fn GetNumOutputs(&self) -> u32;
    fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(&self) -> DXC_OUT_KIND;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDxcResult {}
#[cfg(feature = "Win32_Foundation")]
impl IDxcResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>() -> IDxcResult_Vtbl {
        unsafe extern "system" fn HasOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasOutput(::core::mem::transmute_copy(&dxcoutkind))
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutput(::core::mem::transmute_copy(&dxcoutkind), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        unsafe extern "system" fn GetNumOutputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumOutputs()
        }
        unsafe extern "system" fn GetOutputByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn PrimaryOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrimaryOutput()
        }
        Self {
            base__: IDxcOperationResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasOutput: HasOutput::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetNumOutputs: GetNumOutputs::<Identity, Impl, OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Identity, Impl, OFFSET>,
            PrimaryOutput: PrimaryOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcResult as ::windows::core::ComInterface>::IID || iid == &<IDxcOperationResult as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcUtils_Impl: Sized {
    fn CreateBlobFromBlob(&self, pblob: ::core::option::Option<&IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&self, pdata: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn LoadFile(&self, pfilename: &::windows::core::PCWSTR, pcodepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(&self, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()>;
    fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn BuildArguments(&self, psourcename: &::windows::core::PCWSTR, pentrypoint: &::windows::core::PCWSTR, ptargetprofile: &::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&self, ppdbblob: ::core::option::Option<&IDxcBlob>, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDxcUtils {}
#[cfg(feature = "Win32_System_Com")]
impl IDxcUtils_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>() -> IDxcUtils_Vtbl {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobFromBlob(::windows::core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlobFromPinned(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveToBlob(::core::mem::transmute_copy(&pdata), ::windows::core::from_raw_borrowed(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBlob(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadFile(::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&pcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateReadOnlyStreamFromBlob(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDefaultIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlobAsUtf8(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBlobAsUtf16(::windows::core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDxilContainerPart(::core::mem::transmute_copy(&pshader), ::core::mem::transmute_copy(&dxcpart), ::core::mem::transmute_copy(&pppartdata), ::core::mem::transmute_copy(&ppartsizeinbytes)).into()
        }
        unsafe extern "system" fn CreateReflection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateReflection(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvreflection)).into()
        }
        unsafe extern "system" fn BuildArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BuildArguments(::core::mem::transmute(&psourcename), ::core::mem::transmute(&pentrypoint), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppargs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPDBContents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbblob: *mut ::core::ffi::c_void, pphash: *mut *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPDBContents(::windows::core::from_raw_borrowed(&ppdbblob), ::core::mem::transmute_copy(&pphash), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Identity, Impl, OFFSET>,
            MoveToBlob: MoveToBlob::<Identity, Impl, OFFSET>,
            CreateBlob: CreateBlob::<Identity, Impl, OFFSET>,
            LoadFile: LoadFile::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Identity, Impl, OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Identity, Impl, OFFSET>,
            CreateReflection: CreateReflection::<Identity, Impl, OFFSET>,
            BuildArguments: BuildArguments::<Identity, Impl, OFFSET>,
            GetPDBContents: GetPDBContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcUtils as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcValidator_Impl: Sized {
    fn Validate(&self, pshader: ::core::option::Option<&IDxcBlob>, flags: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
impl ::windows::core::RuntimeName for IDxcValidator {}
impl IDxcValidator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: isize>() -> IDxcValidator_Vtbl {
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Validate(::windows::core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Validate: Validate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcValidator2_Impl: Sized + IDxcValidator_Impl {
    fn ValidateWithDebug(&self, pshader: ::core::option::Option<&IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::core::Result<IDxcOperationResult>;
}
impl ::windows::core::RuntimeName for IDxcValidator2 {}
impl IDxcValidator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: isize>() -> IDxcValidator2_Vtbl {
        unsafe extern "system" fn ValidateWithDebug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateWithDebug(::windows::core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&poptdebugbitcode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDxcValidator_Vtbl::new::<Identity, Impl, OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator2 as ::windows::core::ComInterface>::IID || iid == &<IDxcValidator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcVersionInfo_Impl: Sized {
    fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetFlags(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDxcVersionInfo {}
impl IDxcVersionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>() -> IDxcVersionInfo_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersion(::core::mem::transmute_copy(&pmajor), ::core::mem::transmute_copy(&pminor)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcVersionInfo2_Impl: Sized + IDxcVersionInfo_Impl {
    fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDxcVersionInfo2 {}
impl IDxcVersionInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>() -> IDxcVersionInfo2_Vtbl {
        unsafe extern "system" fn GetCommitInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCommitInfo(::core::mem::transmute_copy(&pcommitcount), ::core::mem::transmute_copy(&pcommithash)).into()
        }
        Self { base__: IDxcVersionInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetCommitInfo: GetCommitInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as ::windows::core::ComInterface>::IID || iid == &<IDxcVersionInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"implement\"`*"]
pub trait IDxcVersionInfo3_Impl: Sized {
    fn GetCustomVersionString(&self) -> ::windows::core::Result<*mut i8>;
}
impl ::windows::core::RuntimeName for IDxcVersionInfo3 {}
impl IDxcVersionInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>() -> IDxcVersionInfo3_Vtbl {
        unsafe extern "system" fn GetCustomVersionString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomVersionString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversionstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as ::windows::core::ComInterface>::IID
    }
}
