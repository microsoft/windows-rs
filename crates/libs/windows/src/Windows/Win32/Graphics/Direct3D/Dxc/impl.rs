pub trait IDxcAssembler_Impl: Sized {
    fn AssembleToContainer(&mut self, pshader: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcAssembler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcAssembler_Impl, const OFFSET: isize>() -> IDxcAssembler_Vtbl {
        unsafe extern "system" fn AssembleToContainer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcAssembler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AssembleToContainer(::core::mem::transmute(&pshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AssembleToContainer: AssembleToContainer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcAssembler as ::windows::core::Interface>::IID
    }
}
pub trait IDxcBlob_Impl: Sized {
    fn GetBufferPointer(&mut self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&mut self) -> usize;
}
impl IDxcBlob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlob_Impl, const OFFSET: isize>() -> IDxcBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferSize()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobEncoding_Impl: Sized + IDxcBlob_Impl {
    fn GetEncoding(&mut self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobEncoding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>() -> IDxcBlobEncoding_Vtbl {
        unsafe extern "system" fn GetEncoding<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEncoding(::core::mem::transmute_copy(&pknown), ::core::mem::transmute_copy(&pcodepage)).into()
        }
        Self { base: IDxcBlob_Vtbl::new::<Identity, Impl, OFFSET>(), GetEncoding: GetEncoding::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as ::windows::core::Interface>::IID || iid == &<IDxcBlob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf16_Impl: Sized + IDxcBlob_Impl + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&mut self) -> super::super::super::Foundation::PWSTR;
    fn GetStringLength(&mut self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf16_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>() -> IDxcBlobUtf16_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringLength()
        }
        Self {
            base: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf16 as ::windows::core::Interface>::IID || iid == &<IDxcBlob as ::windows::core::Interface>::IID || iid == &<IDxcBlobEncoding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf8_Impl: Sized + IDxcBlob_Impl + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&mut self) -> super::super::super::Foundation::PSTR;
    fn GetStringLength(&mut self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>() -> IDxcBlobUtf8_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringLength()
        }
        Self {
            base: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as ::windows::core::Interface>::IID || iid == &<IDxcBlob as ::windows::core::Interface>::IID || iid == &<IDxcBlobEncoding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompiler_Impl: Sized {
    fn Compile(&mut self, psource: &::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: &::core::option::Option<IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Preprocess(&mut self, psource: &::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: &::core::option::Option<IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Disassemble(&mut self, psource: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompiler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler_Impl, const OFFSET: isize>() -> IDxcCompiler_Vtbl {
        unsafe extern "system" fn Compile<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compile(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&pentrypoint), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::core::mem::transmute(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preprocess<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Preprocess(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::core::mem::transmute(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Disassemble(::core::mem::transmute(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisassembly = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompiler2_Impl: Sized + IDxcCompiler_Impl {
    fn CompileWithDebug(&mut self, psource: &::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: &::core::option::Option<IDxcIncludeHandler>, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut super::super::super::Foundation::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompiler2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler2_Impl, const OFFSET: isize>() -> IDxcCompiler2_Vtbl {
        unsafe extern "system" fn CompileWithDebug<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr, ppdebugblobname: *mut super::super::super::Foundation::PWSTR, ppdebugblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .CompileWithDebug(
                    ::core::mem::transmute(&psource),
                    ::core::mem::transmute_copy(&psourcename),
                    ::core::mem::transmute_copy(&pentrypoint),
                    ::core::mem::transmute_copy(&ptargetprofile),
                    ::core::mem::transmute_copy(&parguments),
                    ::core::mem::transmute_copy(&argcount),
                    ::core::mem::transmute_copy(&pdefines),
                    ::core::mem::transmute_copy(&definecount),
                    ::core::mem::transmute(&pincludehandler),
                    ::core::mem::transmute_copy(&ppresult),
                    ::core::mem::transmute_copy(&ppdebugblobname),
                    ::core::mem::transmute_copy(&ppdebugblob),
                )
                .into()
        }
        Self { base: IDxcCompiler_Vtbl::new::<Identity, Impl, OFFSET>(), CompileWithDebug: CompileWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler2 as ::windows::core::Interface>::IID || iid == &<IDxcCompiler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompiler3_Impl: Sized {
    fn Compile(&mut self, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: &::core::option::Option<IDxcIncludeHandler>, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Disassemble(&mut self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompiler3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler3_Impl, const OFFSET: isize>() -> IDxcCompiler3_Vtbl {
        unsafe extern "system" fn Compile<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compile(::core::mem::transmute_copy(&psource), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute(&pincludehandler), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disassemble(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompilerArgs_Impl: Sized {
    fn GetArguments(&mut self) -> *mut super::super::super::Foundation::PWSTR;
    fn GetCount(&mut self) -> u32;
    fn AddArguments(&mut self, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddArgumentsUTF8(&mut self, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddDefines(&mut self, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompilerArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>() -> IDxcCompilerArgs_Vtbl {
        unsafe extern "system" fn GetArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetArguments()
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount()
        }
        unsafe extern "system" fn AddArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddArguments(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddArgumentsUTF8(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddDefines<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDefines(::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetArguments: GetArguments::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddArguments: AddArguments::<Identity, Impl, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, Impl, OFFSET>,
            AddDefines: AddDefines::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDxcContainerBuilder_Impl: Sized {
    fn Load(&mut self, pdxilcontainerheader: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn AddPart(&mut self, fourcc: u32, psource: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn RemovePart(&mut self, fourcc: u32) -> ::windows::core::Result<()>;
    fn SerializeContainer(&mut self) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcContainerBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>() -> IDxcContainerBuilder_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdxilcontainerheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&pdxilcontainerheader)).into()
        }
        unsafe extern "system" fn AddPart<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32, psource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPart(::core::mem::transmute_copy(&fourcc), ::core::mem::transmute(&psource)).into()
        }
        unsafe extern "system" fn RemovePart<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePart(::core::mem::transmute_copy(&fourcc)).into()
        }
        unsafe extern "system" fn SerializeContainer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SerializeContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            AddPart: AddPart::<Identity, Impl, OFFSET>,
            RemovePart: RemovePart::<Identity, Impl, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as ::windows::core::Interface>::IID
    }
}
pub trait IDxcContainerReflection_Impl: Sized {
    fn Load(&mut self, pcontainer: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn GetPartCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPartKind(&mut self, idx: u32) -> ::windows::core::Result<u32>;
    fn GetPartContent(&mut self, idx: u32) -> ::windows::core::Result<IDxcBlob>;
    fn FindFirstPartKind(&mut self, kind: u32) -> ::windows::core::Result<u32>;
    fn GetPartReflection(&mut self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDxcContainerReflection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>() -> IDxcContainerReflection_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontainer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&pcontainer)).into()
        }
        unsafe extern "system" fn GetPartCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPartCount() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartKind<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPartKind(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartContent<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPartContent(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindFirstPartKind(::core::mem::transmute_copy(&kind)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartReflection<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPartReflection(::core::mem::transmute_copy(&idx), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            GetPartCount: GetPartCount::<Identity, Impl, OFFSET>,
            GetPartKind: GetPartKind::<Identity, Impl, OFFSET>,
            GetPartContent: GetPartContent::<Identity, Impl, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, Impl, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerReflection as ::windows::core::Interface>::IID
    }
}
pub trait IDxcExtraOutputs_Impl: Sized {
    fn GetOutputCount(&mut self) -> u32;
    fn GetOutput(&mut self, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
}
impl IDxcExtraOutputs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>() -> IDxcExtraOutputs_Vtbl {
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows::core::RawPtr, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputtype), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcIncludeHandler_Impl: Sized {
    fn LoadSource(&mut self, pfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IDxcBlob>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcIncludeHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>() -> IDxcIncludeHandler_Vtbl {
        unsafe extern "system" fn LoadSource<Identity: ::windows::core::IUnknownImpl, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, ppincludesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadSource(::core::mem::transmute_copy(&pfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppincludesource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LoadSource: LoadSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDxcLibrary_Impl: Sized {
    fn SetMalloc(&mut self, pmalloc: &::core::option::Option<super::super::super::System::Com::IMalloc>) -> ::windows::core::Result<()>;
    fn CreateBlobFromBlob(&mut self, pblob: &::core::option::Option<IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&mut self, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&mut self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&mut self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&mut self, ptext: *const ::core::ffi::c_void, pimalloc: &::core::option::Option<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&mut self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDxcLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>() -> IDxcLibrary_Vtbl {
        unsafe extern "system" fn SetMalloc<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMalloc(::core::mem::transmute(&pmalloc)).into()
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobFromBlob(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobFromFile(::core::mem::transmute_copy(&pfilename), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobWithEncodingFromPinned(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobWithEncodingOnHeapCopy(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobWithEncodingOnMalloc(::core::mem::transmute_copy(&ptext), ::core::mem::transmute(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStreamFromBlobReadOnly(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBlobAsUtf8(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBlobAsUtf16(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDxcLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcLinker_Impl: Sized {
    fn RegisterLibrary(&mut self, plibname: super::super::super::Foundation::PWSTR, plib: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn Link(&mut self, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcLinker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLinker_Impl, const OFFSET: isize>() -> IDxcLinker_Vtbl {
        unsafe extern "system" fn RegisterLibrary<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibname: super::super::super::Foundation::PWSTR, plib: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterLibrary(::core::mem::transmute_copy(&plibname), ::core::mem::transmute(&plib)).into()
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&pentryname), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&plibnames), ::core::mem::transmute_copy(&libcount), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterLibrary: RegisterLibrary::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcLinker as ::windows::core::Interface>::IID
    }
}
pub trait IDxcOperationResult_Impl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetResult(&mut self) -> ::windows::core::Result<IDxcBlob>;
    fn GetErrorBuffer(&mut self) -> ::windows::core::Result<IDxcBlobEncoding>;
}
impl IDxcOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOperationResult_Impl, const OFFSET: isize>() -> IDxcOperationResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResult() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *pperrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcOptimizer_Impl: Sized {
    fn GetAvailablePassCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAvailablePass(&mut self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&mut self, pblob: &::core::option::Option<IDxcBlob>, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcOptimizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizer_Impl, const OFFSET: isize>() -> IDxcOptimizer_Vtbl {
        unsafe extern "system" fn GetAvailablePassCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAvailablePassCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailablePass<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAvailablePass(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunOptimizer<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::core::RawPtr, ppoutputtext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunOptimizer(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&ppoptions), ::core::mem::transmute_copy(&optioncount), ::core::mem::transmute_copy(&poutputmodule), ::core::mem::transmute_copy(&ppoutputtext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Identity, Impl, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, Impl, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcOptimizerPass_Impl: Sized {
    fn GetOptionName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetOptionArgCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetOptionArgName(&mut self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetOptionArgDescription(&mut self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcOptimizerPass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>() -> IDxcOptimizerPass_Vtbl {
        unsafe extern "system" fn GetOptionName<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgName<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionArgName(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptionArgDescription(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOptionName: GetOptionName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, Impl, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, Impl, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcPdbUtils_Impl: Sized {
    fn Load(&mut self, ppdbordxil: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn GetSourceCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSource(&mut self, uindex: u32) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn GetSourceName(&mut self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetFlagCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetFlag(&mut self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetArgCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetArg(&mut self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetArgPairCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetArgPair(&mut self, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDefineCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetDefine(&mut self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetTargetProfile(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetEntryPoint(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetMainFileName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetHash(&mut self) -> ::windows::core::Result<IDxcBlob>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn IsFullPDB(&mut self) -> super::super::super::Foundation::BOOL;
    fn GetFullPDB(&mut self) -> ::windows::core::Result<IDxcBlob>;
    fn GetVersionInfo(&mut self) -> ::windows::core::Result<IDxcVersionInfo>;
    fn SetCompiler(&mut self, pcompiler: &::core::option::Option<IDxcCompiler3>) -> ::windows::core::Result<()>;
    fn CompileForFullPDB(&mut self) -> ::windows::core::Result<IDxcResult>;
    fn OverrideArgs(&mut self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()>;
    fn OverrideRootSignature(&mut self, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcPdbUtils_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>() -> IDxcPdbUtils_Vtbl {
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbordxil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppdbordxil)).into()
        }
        unsafe extern "system" fn GetSourceCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSource(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceName<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceName(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlagCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlagCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlag<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlag(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArg<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetArg(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPairCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetArgPairCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPair<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetArgPair(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetDefineCount<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefineCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefine<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefine(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProfile<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTargetProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryPoint<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMainFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHash<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHash() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullPDB<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsFullPDB()
        }
        unsafe extern "system" fn GetFullPDB<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfullpdb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfullpdb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppversioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppversioninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompiler<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompiler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompiler(::core::mem::transmute(&pcompiler)).into()
        }
        unsafe extern "system" fn CompileForFullPDB<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompileForFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideArgs<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OverrideArgs(::core::mem::transmute_copy(&pargpairs), ::core::mem::transmute_copy(&unumargpairs)).into()
        }
        unsafe extern "system" fn OverrideRootSignature<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OverrideRootSignature(::core::mem::transmute_copy(&prootsignature)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDxcPdbUtils as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcResult_Impl: Sized + IDxcOperationResult_Impl {
    fn HasOutput(&mut self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL;
    fn GetOutput(&mut self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
    fn GetNumOutputs(&mut self) -> u32;
    fn GetOutputByIndex(&mut self, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(&mut self) -> DXC_OUT_KIND;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>() -> IDxcResult_Vtbl {
        unsafe extern "system" fn HasOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasOutput(::core::mem::transmute_copy(&dxcoutkind))
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&dxcoutkind), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        unsafe extern "system" fn GetNumOutputs<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumOutputs()
        }
        unsafe extern "system" fn GetOutputByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn PrimaryOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrimaryOutput()
        }
        Self {
            base: IDxcOperationResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasOutput: HasOutput::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetNumOutputs: GetNumOutputs::<Identity, Impl, OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Identity, Impl, OFFSET>,
            PrimaryOutput: PrimaryOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcResult as ::windows::core::Interface>::IID || iid == &<IDxcOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDxcUtils_Impl: Sized {
    fn CreateBlobFromBlob(&mut self, pblob: &::core::option::Option<IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&mut self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&mut self, pdata: *const ::core::ffi::c_void, pimalloc: &::core::option::Option<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&mut self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn LoadFile(&mut self, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(&mut self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(&mut self, pblob: &::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(&mut self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()>;
    fn CreateReflection(&mut self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn BuildArguments(&mut self, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&mut self, ppdbblob: &::core::option::Option<IDxcBlob>, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDxcUtils_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>() -> IDxcUtils_Vtbl {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobFromBlob(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlobFromPinned(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToBlob<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveToBlob(::core::mem::transmute_copy(&pdata), ::core::mem::transmute(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlob<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateBlob(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFile<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadFile(::core::mem::transmute_copy(&pfilename), ::core::mem::transmute_copy(&pcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateReadOnlyStreamFromBlob(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDefaultIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBlobAsUtf8(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBlobAsUtf16(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDxilContainerPart(::core::mem::transmute_copy(&pshader), ::core::mem::transmute_copy(&dxcpart), ::core::mem::transmute_copy(&pppartdata), ::core::mem::transmute_copy(&ppartsizeinbytes)).into()
        }
        unsafe extern "system" fn CreateReflection<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateReflection(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvreflection)).into()
        }
        unsafe extern "system" fn BuildArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuildArguments(::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&pentrypoint), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppargs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPDBContents<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbblob: ::windows::core::RawPtr, pphash: *mut ::windows::core::RawPtr, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPDBContents(::core::mem::transmute(&ppdbblob), ::core::mem::transmute_copy(&pphash), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDxcUtils as ::windows::core::Interface>::IID
    }
}
pub trait IDxcValidator_Impl: Sized {
    fn Validate(&mut self, pshader: &::core::option::Option<IDxcBlob>, flags: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcValidator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidator_Impl, const OFFSET: isize>() -> IDxcValidator_Vtbl {
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Validate(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Validate: Validate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator as ::windows::core::Interface>::IID
    }
}
pub trait IDxcValidator2_Impl: Sized + IDxcValidator_Impl {
    fn ValidateWithDebug(&mut self, pshader: &::core::option::Option<IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcValidator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidator2_Impl, const OFFSET: isize>() -> IDxcValidator2_Vtbl {
        unsafe extern "system" fn ValidateWithDebug<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ValidateWithDebug(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&poptdebugbitcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDxcValidator_Vtbl::new::<Identity, Impl, OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator2 as ::windows::core::Interface>::IID || iid == &<IDxcValidator as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfo_Impl: Sized {
    fn GetVersion(&mut self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self) -> ::windows::core::Result<u32>;
}
impl IDxcVersionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>() -> IDxcVersionInfo_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pmajor), ::core::mem::transmute_copy(&pminor)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfo2_Impl: Sized + IDxcVersionInfo_Impl {
    fn GetCommitInfo(&mut self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()>;
}
impl IDxcVersionInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>() -> IDxcVersionInfo2_Vtbl {
        unsafe extern "system" fn GetCommitInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCommitInfo(::core::mem::transmute_copy(&pcommitcount), ::core::mem::transmute_copy(&pcommithash)).into()
        }
        Self { base: IDxcVersionInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetCommitInfo: GetCommitInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as ::windows::core::Interface>::IID || iid == &<IDxcVersionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfo3_Impl: Sized {
    fn GetCustomVersionString(&mut self) -> ::windows::core::Result<*mut i8>;
}
impl IDxcVersionInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>() -> IDxcVersionInfo3_Vtbl {
        unsafe extern "system" fn GetCustomVersionString<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCustomVersionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pversionstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as ::windows::core::Interface>::IID
    }
}
