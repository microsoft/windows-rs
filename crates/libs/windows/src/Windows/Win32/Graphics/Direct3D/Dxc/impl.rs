pub trait IDxcAssemblerImpl: Sized {
    fn AssembleToContainer(&mut self, pshader: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcAssemblerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcAssemblerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcAssemblerVtbl {
        unsafe extern "system" fn AssembleToContainer<Impl: IDxcAssemblerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssembleToContainer(::core::mem::transmute(&pshader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AssembleToContainer: AssembleToContainer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcAssembler as ::windows::core::Interface>::IID
    }
}
pub trait IDxcBlobImpl: Sized {
    fn GetBufferPointer(&mut self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&mut self) -> usize;
}
impl IDxcBlobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcBlobVtbl {
        unsafe extern "system" fn GetBufferPointer<Impl: IDxcBlobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Impl: IDxcBlobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferSize()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Impl, IMPL_OFFSET>,
            GetBufferSize: GetBufferSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlob as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobEncodingImpl: Sized + IDxcBlobImpl {
    fn GetEncoding(&mut self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobEncodingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobEncodingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcBlobEncodingVtbl {
        unsafe extern "system" fn GetEncoding<Impl: IDxcBlobEncodingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEncoding(::core::mem::transmute_copy(&pknown), ::core::mem::transmute_copy(&pcodepage)).into()
        }
        Self { base: IDxcBlobVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetEncoding: GetEncoding::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf16Impl: Sized + IDxcBlobImpl + IDxcBlobEncodingImpl {
    fn GetStringPointer(&mut self) -> super::super::super::Foundation::PWSTR;
    fn GetStringLength(&mut self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf16Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf16Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcBlobUtf16Vtbl {
        unsafe extern "system" fn GetStringPointer<Impl: IDxcBlobUtf16Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Impl: IDxcBlobUtf16Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringLength()
        }
        Self {
            base: IDxcBlobEncodingVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStringPointer: GetStringPointer::<Impl, IMPL_OFFSET>,
            GetStringLength: GetStringLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf16 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf8Impl: Sized + IDxcBlobImpl + IDxcBlobEncodingImpl {
    fn GetStringPointer(&mut self) -> super::super::super::Foundation::PSTR;
    fn GetStringLength(&mut self) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcBlobUtf8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcBlobUtf8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcBlobUtf8Vtbl {
        unsafe extern "system" fn GetStringPointer<Impl: IDxcBlobUtf8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringPointer()
        }
        unsafe extern "system" fn GetStringLength<Impl: IDxcBlobUtf8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringLength()
        }
        Self {
            base: IDxcBlobEncodingVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStringPointer: GetStringPointer::<Impl, IMPL_OFFSET>,
            GetStringLength: GetStringLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompilerImpl: Sized {
    fn Compile(&mut self, psource: ::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Preprocess(&mut self, psource: ::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<IDxcIncludeHandler>) -> ::windows::core::Result<IDxcOperationResult>;
    fn Disassemble(&mut self, psource: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompilerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcCompilerVtbl {
        unsafe extern "system" fn Compile<Impl: IDxcCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compile(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&pentrypoint), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::core::mem::transmute(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preprocess<Impl: IDxcCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Preprocess(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::core::mem::transmute(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Impl: IDxcCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disassemble(::core::mem::transmute(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisassembly = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compile: Compile::<Impl, IMPL_OFFSET>,
            Preprocess: Preprocess::<Impl, IMPL_OFFSET>,
            Disassemble: Disassemble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompiler2Impl: Sized + IDxcCompilerImpl {
    fn CompileWithDebug(&mut self, psource: ::core::option::Option<IDxcBlob>, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<IDxcIncludeHandler>, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut super::super::super::Foundation::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompiler2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcCompiler2Vtbl {
        unsafe extern "system" fn CompileWithDebug<Impl: IDxcCompiler2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr, ppdebugblobname: *mut super::super::super::Foundation::PWSTR, ppdebugblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self { base: IDxcCompilerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CompileWithDebug: CompileWithDebug::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompiler3Impl: Sized {
    fn Compile(&mut self, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::core::option::Option<IDxcIncludeHandler>, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Disassemble(&mut self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompiler3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompiler3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcCompiler3Vtbl {
        unsafe extern "system" fn Compile<Impl: IDxcCompiler3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compile(::core::mem::transmute_copy(&psource), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute(&pincludehandler), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Disassemble<Impl: IDxcCompiler3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disassemble(::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Compile: Compile::<Impl, IMPL_OFFSET>,
            Disassemble: Disassemble::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompiler3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcCompilerArgsImpl: Sized {
    fn GetArguments(&mut self) -> *mut super::super::super::Foundation::PWSTR;
    fn GetCount(&mut self) -> u32;
    fn AddArguments(&mut self, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddArgumentsUTF8(&mut self, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::Result<()>;
    fn AddDefines(&mut self, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcCompilerArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcCompilerArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcCompilerArgsVtbl {
        unsafe extern "system" fn GetArguments<Impl: IDxcCompilerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetArguments()
        }
        unsafe extern "system" fn GetCount<Impl: IDxcCompilerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount()
        }
        unsafe extern "system" fn AddArguments<Impl: IDxcCompilerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddArguments(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddArgumentsUTF8<Impl: IDxcCompilerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddArgumentsUTF8(::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddDefines<Impl: IDxcCompilerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDefines(::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetArguments: GetArguments::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            AddArguments: AddArguments::<Impl, IMPL_OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Impl, IMPL_OFFSET>,
            AddDefines: AddDefines::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDxcContainerBuilderImpl: Sized {
    fn Load(&mut self, pdxilcontainerheader: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn AddPart(&mut self, fourcc: u32, psource: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn RemovePart(&mut self, fourcc: u32) -> ::windows::core::Result<()>;
    fn SerializeContainer(&mut self) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcContainerBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcContainerBuilderVtbl {
        unsafe extern "system" fn Load<Impl: IDxcContainerBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdxilcontainerheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&pdxilcontainerheader)).into()
        }
        unsafe extern "system" fn AddPart<Impl: IDxcContainerBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32, psource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPart(::core::mem::transmute_copy(&fourcc), ::core::mem::transmute(&psource)).into()
        }
        unsafe extern "system" fn RemovePart<Impl: IDxcContainerBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePart(::core::mem::transmute_copy(&fourcc)).into()
        }
        unsafe extern "system" fn SerializeContainer<Impl: IDxcContainerBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerializeContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Load: Load::<Impl, IMPL_OFFSET>,
            AddPart: AddPart::<Impl, IMPL_OFFSET>,
            RemovePart: RemovePart::<Impl, IMPL_OFFSET>,
            SerializeContainer: SerializeContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as ::windows::core::Interface>::IID
    }
}
pub trait IDxcContainerReflectionImpl: Sized {
    fn Load(&mut self, pcontainer: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn GetPartCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPartKind(&mut self, idx: u32) -> ::windows::core::Result<u32>;
    fn GetPartContent(&mut self, idx: u32) -> ::windows::core::Result<IDxcBlob>;
    fn FindFirstPartKind(&mut self, kind: u32) -> ::windows::core::Result<u32>;
    fn GetPartReflection(&mut self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDxcContainerReflectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcContainerReflectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcContainerReflectionVtbl {
        unsafe extern "system" fn Load<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontainer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&pcontainer)).into()
        }
        unsafe extern "system" fn GetPartCount<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartCount() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartKind<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartKind(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartContent<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartContent(::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstPartKind(::core::mem::transmute_copy(&kind)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartReflection<Impl: IDxcContainerReflectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPartReflection(::core::mem::transmute_copy(&idx), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Load: Load::<Impl, IMPL_OFFSET>,
            GetPartCount: GetPartCount::<Impl, IMPL_OFFSET>,
            GetPartKind: GetPartKind::<Impl, IMPL_OFFSET>,
            GetPartContent: GetPartContent::<Impl, IMPL_OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Impl, IMPL_OFFSET>,
            GetPartReflection: GetPartReflection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcContainerReflection as ::windows::core::Interface>::IID
    }
}
pub trait IDxcExtraOutputsImpl: Sized {
    fn GetOutputCount(&mut self) -> u32;
    fn GetOutput(&mut self, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
}
impl IDxcExtraOutputsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcExtraOutputsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcExtraOutputsVtbl {
        unsafe extern "system" fn GetOutputCount<Impl: IDxcExtraOutputsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputCount()
        }
        unsafe extern "system" fn GetOutput<Impl: IDxcExtraOutputsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows::core::RawPtr, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputtype), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            GetOutput: GetOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcIncludeHandlerImpl: Sized {
    fn LoadSource(&mut self, pfilename: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IDxcBlob>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcIncludeHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcIncludeHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcIncludeHandlerVtbl {
        unsafe extern "system" fn LoadSource<Impl: IDxcIncludeHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, ppincludesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadSource(::core::mem::transmute_copy(&pfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppincludesource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LoadSource: LoadSource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDxcLibraryImpl: Sized {
    fn SetMalloc(&mut self, pmalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>) -> ::windows::core::Result<()>;
    fn CreateBlobFromBlob(&mut self, pblob: ::core::option::Option<IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&mut self, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&mut self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&mut self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&mut self, ptext: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&mut self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobEncoding>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDxcLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcLibraryVtbl {
        unsafe extern "system" fn SetMalloc<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMalloc(::core::mem::transmute(&pmalloc)).into()
        }
        unsafe extern "system" fn CreateBlobFromBlob<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromBlob(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromFile(::core::mem::transmute_copy(&pfilename), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingFromPinned(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingOnHeapCopy(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingOnMalloc(::core::mem::transmute_copy(&ptext), ::core::mem::transmute(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStreamFromBlobReadOnly(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf8(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Impl: IDxcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf16(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMalloc: SetMalloc::<Impl, IMPL_OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Impl, IMPL_OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Impl, IMPL_OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Impl, IMPL_OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Impl, IMPL_OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Impl, IMPL_OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Impl, IMPL_OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Impl, IMPL_OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Impl, IMPL_OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcLinkerImpl: Sized {
    fn RegisterLibrary(&mut self, plibname: super::super::super::Foundation::PWSTR, plib: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
    fn Link(&mut self, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcLinkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcLinkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcLinkerVtbl {
        unsafe extern "system" fn RegisterLibrary<Impl: IDxcLinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plibname: super::super::super::Foundation::PWSTR, plib: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterLibrary(::core::mem::transmute_copy(&plibname), ::core::mem::transmute(&plib)).into()
        }
        unsafe extern "system" fn Link<Impl: IDxcLinkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link(::core::mem::transmute_copy(&pentryname), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&plibnames), ::core::mem::transmute_copy(&libcount), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterLibrary: RegisterLibrary::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcLinker as ::windows::core::Interface>::IID
    }
}
pub trait IDxcOperationResultImpl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetResult(&mut self) -> ::windows::core::Result<IDxcBlob>;
    fn GetErrorBuffer(&mut self) -> ::windows::core::Result<IDxcBlobEncoding>;
}
impl IDxcOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcOperationResultVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDxcOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Impl: IDxcOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResult() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Impl: IDxcOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *pperrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetResult: GetResult::<Impl, IMPL_OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcOptimizerImpl: Sized {
    fn GetAvailablePassCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAvailablePass(&mut self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&mut self, pblob: ::core::option::Option<IDxcBlob>, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcOptimizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcOptimizerVtbl {
        unsafe extern "system" fn GetAvailablePassCount<Impl: IDxcOptimizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailablePassCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailablePass<Impl: IDxcOptimizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailablePass(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunOptimizer<Impl: IDxcOptimizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::core::RawPtr, ppoutputtext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunOptimizer(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&ppoptions), ::core::mem::transmute_copy(&optioncount), ::core::mem::transmute_copy(&poutputmodule), ::core::mem::transmute_copy(&ppoutputtext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Impl, IMPL_OFFSET>,
            GetAvailablePass: GetAvailablePass::<Impl, IMPL_OFFSET>,
            RunOptimizer: RunOptimizer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcOptimizerPassImpl: Sized {
    fn GetOptionName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetOptionArgCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetOptionArgName(&mut self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetOptionArgDescription(&mut self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcOptimizerPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcOptimizerPassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcOptimizerPassVtbl {
        unsafe extern "system" fn GetOptionName<Impl: IDxcOptimizerPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IDxcOptimizerPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Impl: IDxcOptimizerPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgName<Impl: IDxcOptimizerPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionArgName(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Impl: IDxcOptimizerPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionArgDescription(::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOptionName: GetOptionName::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Impl, IMPL_OFFSET>,
            GetOptionArgName: GetOptionArgName::<Impl, IMPL_OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcPdbUtilsImpl: Sized {
    fn Load(&mut self, ppdbordxil: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
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
    fn SetCompiler(&mut self, pcompiler: ::core::option::Option<IDxcCompiler3>) -> ::windows::core::Result<()>;
    fn CompileForFullPDB(&mut self) -> ::windows::core::Result<IDxcResult>;
    fn OverrideArgs(&mut self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()>;
    fn OverrideRootSignature(&mut self, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcPdbUtilsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcPdbUtilsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcPdbUtilsVtbl {
        unsafe extern "system" fn Load<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbordxil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute(&ppdbordxil)).into()
        }
        unsafe extern "system" fn GetSourceCount<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceName<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceName(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlagCount<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlagCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlag<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlag(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgCount<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArgCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArg<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArg(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPairCount<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetArgPairCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPair<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetArgPair(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetDefineCount<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefineCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefine<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefine(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProfile<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryPoint<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainFileName<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHash<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHash() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsFullPDB()
        }
        unsafe extern "system" fn GetFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfullpdb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfullpdb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionInfo<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppversioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppversioninfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompiler<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompiler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompiler(::core::mem::transmute(&pcompiler)).into()
        }
        unsafe extern "system" fn CompileForFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompileForFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideArgs<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideArgs(::core::mem::transmute_copy(&pargpairs), ::core::mem::transmute_copy(&unumargpairs)).into()
        }
        unsafe extern "system" fn OverrideRootSignature<Impl: IDxcPdbUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideRootSignature(::core::mem::transmute_copy(&prootsignature)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Load: Load::<Impl, IMPL_OFFSET>,
            GetSourceCount: GetSourceCount::<Impl, IMPL_OFFSET>,
            GetSource: GetSource::<Impl, IMPL_OFFSET>,
            GetSourceName: GetSourceName::<Impl, IMPL_OFFSET>,
            GetFlagCount: GetFlagCount::<Impl, IMPL_OFFSET>,
            GetFlag: GetFlag::<Impl, IMPL_OFFSET>,
            GetArgCount: GetArgCount::<Impl, IMPL_OFFSET>,
            GetArg: GetArg::<Impl, IMPL_OFFSET>,
            GetArgPairCount: GetArgPairCount::<Impl, IMPL_OFFSET>,
            GetArgPair: GetArgPair::<Impl, IMPL_OFFSET>,
            GetDefineCount: GetDefineCount::<Impl, IMPL_OFFSET>,
            GetDefine: GetDefine::<Impl, IMPL_OFFSET>,
            GetTargetProfile: GetTargetProfile::<Impl, IMPL_OFFSET>,
            GetEntryPoint: GetEntryPoint::<Impl, IMPL_OFFSET>,
            GetMainFileName: GetMainFileName::<Impl, IMPL_OFFSET>,
            GetHash: GetHash::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            IsFullPDB: IsFullPDB::<Impl, IMPL_OFFSET>,
            GetFullPDB: GetFullPDB::<Impl, IMPL_OFFSET>,
            GetVersionInfo: GetVersionInfo::<Impl, IMPL_OFFSET>,
            SetCompiler: SetCompiler::<Impl, IMPL_OFFSET>,
            CompileForFullPDB: CompileForFullPDB::<Impl, IMPL_OFFSET>,
            OverrideArgs: OverrideArgs::<Impl, IMPL_OFFSET>,
            OverrideRootSignature: OverrideRootSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcPdbUtils as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcResultImpl: Sized + IDxcOperationResultImpl {
    fn HasOutput(&mut self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL;
    fn GetOutput(&mut self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()>;
    fn GetNumOutputs(&mut self) -> u32;
    fn GetOutputByIndex(&mut self, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(&mut self) -> DXC_OUT_KIND;
}
#[cfg(feature = "Win32_Foundation")]
impl IDxcResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcResultVtbl {
        unsafe extern "system" fn HasOutput<Impl: IDxcResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasOutput(::core::mem::transmute_copy(&dxcoutkind))
        }
        unsafe extern "system" fn GetOutput<Impl: IDxcResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutput(::core::mem::transmute_copy(&dxcoutkind), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputname)).into()
        }
        unsafe extern "system" fn GetNumOutputs<Impl: IDxcResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumOutputs()
        }
        unsafe extern "system" fn GetOutputByIndex<Impl: IDxcResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputByIndex(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn PrimaryOutput<Impl: IDxcResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrimaryOutput()
        }
        Self {
            base: IDxcOperationResultVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HasOutput: HasOutput::<Impl, IMPL_OFFSET>,
            GetOutput: GetOutput::<Impl, IMPL_OFFSET>,
            GetNumOutputs: GetNumOutputs::<Impl, IMPL_OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Impl, IMPL_OFFSET>,
            PrimaryOutput: PrimaryOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDxcUtilsImpl: Sized {
    fn CreateBlobFromBlob(&mut self, pblob: ::core::option::Option<IDxcBlob>, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&mut self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&mut self, pdata: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&mut self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn LoadFile(&mut self, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(&mut self) -> ::windows::core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(&mut self, pblob: ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(&mut self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()>;
    fn CreateReflection(&mut self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn BuildArguments(&mut self, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&mut self, ppdbblob: ::core::option::Option<IDxcBlob>, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDxcUtilsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcUtilsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcUtilsVtbl {
        unsafe extern "system" fn CreateBlobFromBlob<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromBlob(::core::mem::transmute(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromPinned(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToBlob<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveToBlob(::core::mem::transmute_copy(&pdata), ::core::mem::transmute(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlob<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBlob(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFile<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFile(::core::mem::transmute_copy(&pfilename), ::core::mem::transmute_copy(&pcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReadOnlyStreamFromBlob(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDefaultIncludeHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf8(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf16(::core::mem::transmute(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *pblobencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDxilContainerPart(::core::mem::transmute_copy(&pshader), ::core::mem::transmute_copy(&dxcpart), ::core::mem::transmute_copy(&pppartdata), ::core::mem::transmute_copy(&ppartsizeinbytes)).into()
        }
        unsafe extern "system" fn CreateReflection<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateReflection(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvreflection)).into()
        }
        unsafe extern "system" fn BuildArguments<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildArguments(::core::mem::transmute_copy(&psourcename), ::core::mem::transmute_copy(&pentrypoint), ::core::mem::transmute_copy(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppargs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPDBContents<Impl: IDxcUtilsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdbblob: ::windows::core::RawPtr, pphash: *mut ::windows::core::RawPtr, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPDBContents(::core::mem::transmute(&ppdbblob), ::core::mem::transmute_copy(&pphash), ::core::mem::transmute_copy(&ppcontainer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateBlobFromBlob: CreateBlobFromBlob::<Impl, IMPL_OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Impl, IMPL_OFFSET>,
            MoveToBlob: MoveToBlob::<Impl, IMPL_OFFSET>,
            CreateBlob: CreateBlob::<Impl, IMPL_OFFSET>,
            LoadFile: LoadFile::<Impl, IMPL_OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Impl, IMPL_OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Impl, IMPL_OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Impl, IMPL_OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Impl, IMPL_OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Impl, IMPL_OFFSET>,
            CreateReflection: CreateReflection::<Impl, IMPL_OFFSET>,
            BuildArguments: BuildArguments::<Impl, IMPL_OFFSET>,
            GetPDBContents: GetPDBContents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcUtils as ::windows::core::Interface>::IID
    }
}
pub trait IDxcValidatorImpl: Sized {
    fn Validate(&mut self, pshader: ::core::option::Option<IDxcBlob>, flags: u32) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcValidatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcValidatorVtbl {
        unsafe extern "system" fn Validate<Impl: IDxcValidatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Validate: Validate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator as ::windows::core::Interface>::IID
    }
}
pub trait IDxcValidator2Impl: Sized + IDxcValidatorImpl {
    fn ValidateWithDebug(&mut self, pshader: ::core::option::Option<IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::core::Result<IDxcOperationResult>;
}
impl IDxcValidator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcValidator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcValidator2Vtbl {
        unsafe extern "system" fn ValidateWithDebug<Impl: IDxcValidator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateWithDebug(::core::mem::transmute(&pshader), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&poptdebugbitcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDxcValidatorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcValidator2 as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfoImpl: Sized {
    fn GetVersion(&mut self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self) -> ::windows::core::Result<u32>;
}
impl IDxcVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcVersionInfoVtbl {
        unsafe extern "system" fn GetVersion<Impl: IDxcVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pmajor), ::core::mem::transmute_copy(&pminor)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IDxcVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfo2Impl: Sized + IDxcVersionInfoImpl {
    fn GetCommitInfo(&mut self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()>;
}
impl IDxcVersionInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcVersionInfo2Vtbl {
        unsafe extern "system" fn GetCommitInfo<Impl: IDxcVersionInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCommitInfo(::core::mem::transmute_copy(&pcommitcount), ::core::mem::transmute_copy(&pcommithash)).into()
        }
        Self { base: IDxcVersionInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCommitInfo: GetCommitInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait IDxcVersionInfo3Impl: Sized {
    fn GetCustomVersionString(&mut self) -> ::windows::core::Result<*mut i8>;
}
impl IDxcVersionInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDxcVersionInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDxcVersionInfo3Vtbl {
        unsafe extern "system" fn GetCustomVersionString<Impl: IDxcVersionInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomVersionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pversionstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as ::windows::core::Interface>::IID
    }
}
