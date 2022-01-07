pub trait IDxcAssemblerImpl: Sized {
    fn AssembleToContainer();
}
impl ::windows::core::RuntimeName for IDxcAssembler {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcAssembler";
}
impl IDxcAssemblerVtbl {
    pub const fn new<Impl: IDxcAssemblerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcAssemblerVtbl {
        unsafe extern "system" fn AssembleToContainer<Impl: IDxcAssemblerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssembleToContainer(&*(&pshader as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcAssembler>, base.5, AssembleToContainer::<Impl, OFFSET>)
    }
}
pub trait IDxcBlobImpl: Sized {
    fn GetBufferPointer();
    fn GetBufferSize();
}
impl ::windows::core::RuntimeName for IDxcBlob {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcBlob";
}
impl IDxcBlobVtbl {
    pub const fn new<Impl: IDxcBlobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcBlobVtbl {
        unsafe extern "system" fn GetBufferPointer<Impl: IDxcBlobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferPointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferSize<Impl: IDxcBlobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcBlob>, base.5, GetBufferPointer::<Impl, OFFSET>, GetBufferSize::<Impl, OFFSET>)
    }
}
pub trait IDxcBlobEncodingImpl: Sized + IDxcBlobImpl {
    fn GetEncoding();
}
impl ::windows::core::RuntimeName for IDxcBlobEncoding {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcBlobEncoding";
}
impl IDxcBlobEncodingVtbl {
    pub const fn new<Impl: IDxcBlobEncodingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcBlobEncodingVtbl {
        unsafe extern "system" fn GetEncoding<Impl: IDxcBlobEncodingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEncoding(::core::mem::transmute_copy(&pknown), ::core::mem::transmute_copy(&pcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcBlobEncoding>, base.5, GetEncoding::<Impl, OFFSET>)
    }
}
pub trait IDxcBlobUtf16Impl: Sized + IDxcBlobEncodingImpl + IDxcBlobImpl {
    fn GetStringPointer();
    fn GetStringLength();
}
impl ::windows::core::RuntimeName for IDxcBlobUtf16 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcBlobUtf16";
}
impl IDxcBlobUtf16Vtbl {
    pub const fn new<Impl: IDxcBlobUtf16Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcBlobUtf16Vtbl {
        unsafe extern "system" fn GetStringPointer<Impl: IDxcBlobUtf16Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringPointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringLength<Impl: IDxcBlobUtf16Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcBlobUtf16>, base.5, GetStringPointer::<Impl, OFFSET>, GetStringLength::<Impl, OFFSET>)
    }
}
pub trait IDxcBlobUtf8Impl: Sized + IDxcBlobEncodingImpl + IDxcBlobImpl {
    fn GetStringPointer();
    fn GetStringLength();
}
impl ::windows::core::RuntimeName for IDxcBlobUtf8 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcBlobUtf8";
}
impl IDxcBlobUtf8Vtbl {
    pub const fn new<Impl: IDxcBlobUtf8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcBlobUtf8Vtbl {
        unsafe extern "system" fn GetStringPointer<Impl: IDxcBlobUtf8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::PSTR {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringPointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringLength<Impl: IDxcBlobUtf8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStringLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcBlobUtf8>, base.5, GetStringPointer::<Impl, OFFSET>, GetStringLength::<Impl, OFFSET>)
    }
}
pub trait IDxcCompilerImpl: Sized {
    fn Compile();
    fn Preprocess();
    fn Disassemble();
}
impl ::windows::core::RuntimeName for IDxcCompiler {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcCompiler";
}
impl IDxcCompilerVtbl {
    pub const fn new<Impl: IDxcCompilerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcCompilerVtbl {
        unsafe extern "system" fn Compile<Impl: IDxcCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compile(
                &*(&psource as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pentrypoint as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptargetprofile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                &*(&pdefines as *const <DxcDefine as ::windows::core::Abi>::Abi as *const <DxcDefine as ::windows::core::DefaultType>::DefaultType),
                definecount,
                &*(&pincludehandler as *const <IDxcIncludeHandler as ::windows::core::Abi>::Abi as *const <IDxcIncludeHandler as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preprocess<Impl: IDxcCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preprocess(
                &*(&psource as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                &*(&pdefines as *const <DxcDefine as ::windows::core::Abi>::Abi as *const <DxcDefine as ::windows::core::DefaultType>::DefaultType),
                definecount,
                &*(&pincludehandler as *const <IDxcIncludeHandler as ::windows::core::Abi>::Abi as *const <IDxcIncludeHandler as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Impl: IDxcCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disassemble(&*(&psource as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdisassembly)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcCompiler>, base.5, Compile::<Impl, OFFSET>, Preprocess::<Impl, OFFSET>, Disassemble::<Impl, OFFSET>)
    }
}
pub trait IDxcCompiler2Impl: Sized + IDxcCompilerImpl {
    fn CompileWithDebug();
}
impl ::windows::core::RuntimeName for IDxcCompiler2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcCompiler2";
}
impl IDxcCompiler2Vtbl {
    pub const fn new<Impl: IDxcCompiler2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcCompiler2Vtbl {
        unsafe extern "system" fn CompileWithDebug<Impl: IDxcCompiler2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr, ppdebugblobname: *mut super::super::super::Foundation::PWSTR, ppdebugblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileWithDebug(
                &*(&psource as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pentrypoint as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptargetprofile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                &*(&pdefines as *const <DxcDefine as ::windows::core::Abi>::Abi as *const <DxcDefine as ::windows::core::DefaultType>::DefaultType),
                definecount,
                &*(&pincludehandler as *const <IDxcIncludeHandler as ::windows::core::Abi>::Abi as *const <IDxcIncludeHandler as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
                ::core::mem::transmute_copy(&ppdebugblobname),
                ::core::mem::transmute_copy(&ppdebugblob),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcCompiler2>, base.5, CompileWithDebug::<Impl, OFFSET>)
    }
}
pub trait IDxcCompiler3Impl: Sized {
    fn Compile();
    fn Disassemble();
}
impl ::windows::core::RuntimeName for IDxcCompiler3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcCompiler3";
}
impl IDxcCompiler3Vtbl {
    pub const fn new<Impl: IDxcCompiler3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcCompiler3Vtbl {
        unsafe extern "system" fn Compile<Impl: IDxcCompiler3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compile(
                &*(&psource as *const <DxcBuffer as ::windows::core::Abi>::Abi as *const <DxcBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                &*(&pincludehandler as *const <IDxcIncludeHandler as ::windows::core::Abi>::Abi as *const <IDxcIncludeHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Impl: IDxcCompiler3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disassemble(&*(&pobject as *const <DxcBuffer as ::windows::core::Abi>::Abi as *const <DxcBuffer as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcCompiler3>, base.5, Compile::<Impl, OFFSET>, Disassemble::<Impl, OFFSET>)
    }
}
pub trait IDxcCompilerArgsImpl: Sized {
    fn GetArguments();
    fn GetCount();
    fn AddArguments();
    fn AddArgumentsUTF8();
    fn AddDefines();
}
impl ::windows::core::RuntimeName for IDxcCompilerArgs {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcCompilerArgs";
}
impl IDxcCompilerArgsVtbl {
    pub const fn new<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcCompilerArgsVtbl {
        unsafe extern "system" fn GetArguments<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut super::super::super::Foundation::PWSTR {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddArguments<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddArguments(&*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), argcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddArgumentsUTF8<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddArgumentsUTF8(&*(&parguments as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), argcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDefines<Impl: IDxcCompilerArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDefines(&*(&pdefines as *const <DxcDefine as ::windows::core::Abi>::Abi as *const <DxcDefine as ::windows::core::DefaultType>::DefaultType), definecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcCompilerArgs>, base.5, GetArguments::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, AddArguments::<Impl, OFFSET>, AddArgumentsUTF8::<Impl, OFFSET>, AddDefines::<Impl, OFFSET>)
    }
}
pub trait IDxcContainerBuilderImpl: Sized {
    fn Load();
    fn AddPart();
    fn RemovePart();
    fn SerializeContainer();
}
impl ::windows::core::RuntimeName for IDxcContainerBuilder {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcContainerBuilder";
}
impl IDxcContainerBuilderVtbl {
    pub const fn new<Impl: IDxcContainerBuilderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcContainerBuilderVtbl {
        unsafe extern "system" fn Load<Impl: IDxcContainerBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdxilcontainerheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pdxilcontainerheader as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPart<Impl: IDxcContainerBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fourcc: u32, psource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPart(fourcc, &*(&psource as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePart<Impl: IDxcContainerBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemovePart(fourcc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeContainer<Impl: IDxcContainerBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerializeContainer(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcContainerBuilder>, base.5, Load::<Impl, OFFSET>, AddPart::<Impl, OFFSET>, RemovePart::<Impl, OFFSET>, SerializeContainer::<Impl, OFFSET>)
    }
}
pub trait IDxcContainerReflectionImpl: Sized {
    fn Load();
    fn GetPartCount();
    fn GetPartKind();
    fn GetPartContent();
    fn FindFirstPartKind();
    fn GetPartReflection();
}
impl ::windows::core::RuntimeName for IDxcContainerReflection {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcContainerReflection";
}
impl IDxcContainerReflectionVtbl {
    pub const fn new<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcContainerReflectionVtbl {
        unsafe extern "system" fn Load<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontainer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Load(&*(&pcontainer as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartCount<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartCount(::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartKind<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartKind(idx, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartContent<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartContent(idx, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindFirstPartKind(kind, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartReflection<Impl: IDxcContainerReflectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartReflection(idx, &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvobject as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcContainerReflection>, base.5, Load::<Impl, OFFSET>, GetPartCount::<Impl, OFFSET>, GetPartKind::<Impl, OFFSET>, GetPartContent::<Impl, OFFSET>, FindFirstPartKind::<Impl, OFFSET>, GetPartReflection::<Impl, OFFSET>)
    }
}
pub trait IDxcExtraOutputsImpl: Sized {
    fn GetOutputCount();
    fn GetOutput();
}
impl ::windows::core::RuntimeName for IDxcExtraOutputs {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcExtraOutputs";
}
impl IDxcExtraOutputsVtbl {
    pub const fn new<Impl: IDxcExtraOutputsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcExtraOutputsVtbl {
        unsafe extern "system" fn GetOutputCount<Impl: IDxcExtraOutputsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutput<Impl: IDxcExtraOutputsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows::core::RawPtr, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutput(uindex, &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputtype), ::core::mem::transmute_copy(&ppoutputname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcExtraOutputs>, base.5, GetOutputCount::<Impl, OFFSET>, GetOutput::<Impl, OFFSET>)
    }
}
pub trait IDxcIncludeHandlerImpl: Sized {
    fn LoadSource();
}
impl ::windows::core::RuntimeName for IDxcIncludeHandler {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcIncludeHandler";
}
impl IDxcIncludeHandlerVtbl {
    pub const fn new<Impl: IDxcIncludeHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcIncludeHandlerVtbl {
        unsafe extern "system" fn LoadSource<Impl: IDxcIncludeHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, ppincludesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadSource(&*(&pfilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppincludesource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcIncludeHandler>, base.5, LoadSource::<Impl, OFFSET>)
    }
}
pub trait IDxcLibraryImpl: Sized {
    fn SetMalloc();
    fn CreateBlobFromBlob();
    fn CreateBlobFromFile();
    fn CreateBlobWithEncodingFromPinned();
    fn CreateBlobWithEncodingOnHeapCopy();
    fn CreateBlobWithEncodingOnMalloc();
    fn CreateIncludeHandler();
    fn CreateStreamFromBlobReadOnly();
    fn GetBlobAsUtf8();
    fn GetBlobAsUtf16();
}
impl ::windows::core::RuntimeName for IDxcLibrary {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcLibrary";
}
impl IDxcLibraryVtbl {
    pub const fn new<Impl: IDxcLibraryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcLibraryVtbl {
        unsafe extern "system" fn SetMalloc<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMalloc(&*(&pmalloc as *const <super::super::super::System::Com::IMalloc as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IMalloc as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromBlob<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromBlob(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), offset, length, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromFile(&*(&pfilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingFromPinned(&*(&ptext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingOnHeapCopy(&*(&ptext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobWithEncodingOnMalloc(&*(&ptext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&pimalloc as *const <super::super::super::System::Com::IMalloc as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IMalloc as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateIncludeHandler(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStreamFromBlobReadOnly(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf8(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Impl: IDxcLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf16(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDxcLibrary>,
            base.5,
            SetMalloc::<Impl, OFFSET>,
            CreateBlobFromBlob::<Impl, OFFSET>,
            CreateBlobFromFile::<Impl, OFFSET>,
            CreateBlobWithEncodingFromPinned::<Impl, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy::<Impl, OFFSET>,
            CreateBlobWithEncodingOnMalloc::<Impl, OFFSET>,
            CreateIncludeHandler::<Impl, OFFSET>,
            CreateStreamFromBlobReadOnly::<Impl, OFFSET>,
            GetBlobAsUtf8::<Impl, OFFSET>,
            GetBlobAsUtf16::<Impl, OFFSET>,
        )
    }
}
pub trait IDxcLinkerImpl: Sized {
    fn RegisterLibrary();
    fn Link();
}
impl ::windows::core::RuntimeName for IDxcLinker {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcLinker";
}
impl IDxcLinkerVtbl {
    pub const fn new<Impl: IDxcLinkerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcLinkerVtbl {
        unsafe extern "system" fn RegisterLibrary<Impl: IDxcLinkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibname: super::super::super::Foundation::PWSTR, plib: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterLibrary(&*(&plibname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&plib as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: IDxcLinkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Link(
                &*(&pentryname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptargetprofile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&plibnames as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                libcount,
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcLinker>, base.5, RegisterLibrary::<Impl, OFFSET>, Link::<Impl, OFFSET>)
    }
}
pub trait IDxcOperationResultImpl: Sized {
    fn GetStatus();
    fn GetResult();
    fn GetErrorBuffer();
}
impl ::windows::core::RuntimeName for IDxcOperationResult {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcOperationResult";
}
impl IDxcOperationResultVtbl {
    pub const fn new<Impl: IDxcOperationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcOperationResultVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDxcOperationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Impl: IDxcOperationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResult(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Impl: IDxcOperationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorBuffer(::core::mem::transmute_copy(&pperrors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcOperationResult>, base.5, GetStatus::<Impl, OFFSET>, GetResult::<Impl, OFFSET>, GetErrorBuffer::<Impl, OFFSET>)
    }
}
pub trait IDxcOptimizerImpl: Sized {
    fn GetAvailablePassCount();
    fn GetAvailablePass();
    fn RunOptimizer();
}
impl ::windows::core::RuntimeName for IDxcOptimizer {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcOptimizer";
}
impl IDxcOptimizerVtbl {
    pub const fn new<Impl: IDxcOptimizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcOptimizerVtbl {
        unsafe extern "system" fn GetAvailablePassCount<Impl: IDxcOptimizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailablePassCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailablePass<Impl: IDxcOptimizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailablePass(index, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunOptimizer<Impl: IDxcOptimizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::core::RawPtr, ppoutputtext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunOptimizer(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), &*(&ppoptions as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), optioncount, ::core::mem::transmute_copy(&poutputmodule), ::core::mem::transmute_copy(&ppoutputtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcOptimizer>, base.5, GetAvailablePassCount::<Impl, OFFSET>, GetAvailablePass::<Impl, OFFSET>, RunOptimizer::<Impl, OFFSET>)
    }
}
pub trait IDxcOptimizerPassImpl: Sized {
    fn GetOptionName();
    fn GetDescription();
    fn GetOptionArgCount();
    fn GetOptionArgName();
    fn GetOptionArgDescription();
}
impl ::windows::core::RuntimeName for IDxcOptimizerPass {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcOptimizerPass";
}
impl IDxcOptimizerPassVtbl {
    pub const fn new<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcOptimizerPassVtbl {
        unsafe extern "system" fn GetOptionName<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionName(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionArgCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgName<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionArgName(argindex, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Impl: IDxcOptimizerPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionArgDescription(argindex, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcOptimizerPass>, base.5, GetOptionName::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, GetOptionArgCount::<Impl, OFFSET>, GetOptionArgName::<Impl, OFFSET>, GetOptionArgDescription::<Impl, OFFSET>)
    }
}
pub trait IDxcPdbUtilsImpl: Sized {
    fn Load();
    fn GetSourceCount();
    fn GetSource();
    fn GetSourceName();
    fn GetFlagCount();
    fn GetFlag();
    fn GetArgCount();
    fn GetArg();
    fn GetArgPairCount();
    fn GetArgPair();
    fn GetDefineCount();
    fn GetDefine();
    fn GetTargetProfile();
    fn GetEntryPoint();
    fn GetMainFileName();
    fn GetHash();
    fn GetName();
    fn IsFullPDB();
    fn GetFullPDB();
    fn GetVersionInfo();
    fn SetCompiler();
    fn CompileForFullPDB();
    fn OverrideArgs();
    fn OverrideRootSignature();
}
impl ::windows::core::RuntimeName for IDxcPdbUtils {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcPdbUtils";
}
impl IDxcPdbUtilsVtbl {
    pub const fn new<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcPdbUtilsVtbl {
        unsafe extern "system" fn Load<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdbordxil: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Load(&*(&ppdbordxil as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCount<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSource(uindex, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceName<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceName(uindex, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlagCount<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlagCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlag<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlag(uindex, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgCount<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetArgCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArg<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetArg(uindex, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPairCount<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetArgPairCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPair<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetArgPair(uindex, ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefineCount<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefineCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefine<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefine(uindex, ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProfile<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTargetProfile(::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryPoint<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEntryPoint(::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainFileName<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMainFileName(::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHash<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHash(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFullPDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfullpdb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFullPDB(::core::mem::transmute_copy(&ppfullpdb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionInfo<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppversioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersionInfo(::core::mem::transmute_copy(&ppversioninfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompiler<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompiler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCompiler(&*(&pcompiler as *const <IDxcCompiler3 as ::windows::core::Abi>::Abi as *const <IDxcCompiler3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompileForFullPDB<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileForFullPDB(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideArgs<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverrideArgs(&*(&pargpairs as *const <DxcArgPair as ::windows::core::Abi>::Abi as *const <DxcArgPair as ::windows::core::DefaultType>::DefaultType), unumargpairs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideRootSignature<Impl: IDxcPdbUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverrideRootSignature(&*(&prootsignature as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDxcPdbUtils>,
            base.5,
            Load::<Impl, OFFSET>,
            GetSourceCount::<Impl, OFFSET>,
            GetSource::<Impl, OFFSET>,
            GetSourceName::<Impl, OFFSET>,
            GetFlagCount::<Impl, OFFSET>,
            GetFlag::<Impl, OFFSET>,
            GetArgCount::<Impl, OFFSET>,
            GetArg::<Impl, OFFSET>,
            GetArgPairCount::<Impl, OFFSET>,
            GetArgPair::<Impl, OFFSET>,
            GetDefineCount::<Impl, OFFSET>,
            GetDefine::<Impl, OFFSET>,
            GetTargetProfile::<Impl, OFFSET>,
            GetEntryPoint::<Impl, OFFSET>,
            GetMainFileName::<Impl, OFFSET>,
            GetHash::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            IsFullPDB::<Impl, OFFSET>,
            GetFullPDB::<Impl, OFFSET>,
            GetVersionInfo::<Impl, OFFSET>,
            SetCompiler::<Impl, OFFSET>,
            CompileForFullPDB::<Impl, OFFSET>,
            OverrideArgs::<Impl, OFFSET>,
            OverrideRootSignature::<Impl, OFFSET>,
        )
    }
}
pub trait IDxcResultImpl: Sized + IDxcOperationResultImpl {
    fn HasOutput();
    fn GetOutput();
    fn GetNumOutputs();
    fn GetOutputByIndex();
    fn PrimaryOutput();
}
impl ::windows::core::RuntimeName for IDxcResult {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcResult";
}
impl IDxcResultVtbl {
    pub const fn new<Impl: IDxcResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcResultVtbl {
        unsafe extern "system" fn HasOutput<Impl: IDxcResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasOutput(dxcoutkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutput<Impl: IDxcResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutput(dxcoutkind, &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumOutputs<Impl: IDxcResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumOutputs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputByIndex<Impl: IDxcResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputByIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryOutput<Impl: IDxcResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcResult>, base.5, HasOutput::<Impl, OFFSET>, GetOutput::<Impl, OFFSET>, GetNumOutputs::<Impl, OFFSET>, GetOutputByIndex::<Impl, OFFSET>, PrimaryOutput::<Impl, OFFSET>)
    }
}
pub trait IDxcUtilsImpl: Sized {
    fn CreateBlobFromBlob();
    fn CreateBlobFromPinned();
    fn MoveToBlob();
    fn CreateBlob();
    fn LoadFile();
    fn CreateReadOnlyStreamFromBlob();
    fn CreateDefaultIncludeHandler();
    fn GetBlobAsUtf8();
    fn GetBlobAsUtf16();
    fn GetDxilContainerPart();
    fn CreateReflection();
    fn BuildArguments();
    fn GetPDBContents();
}
impl ::windows::core::RuntimeName for IDxcUtils {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcUtils";
}
impl IDxcUtilsVtbl {
    pub const fn new<Impl: IDxcUtilsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcUtilsVtbl {
        unsafe extern "system" fn CreateBlobFromBlob<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromBlob(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), offset, length, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlobFromPinned(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToBlob<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoveToBlob(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&pimalloc as *const <super::super::super::System::Com::IMalloc as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IMalloc as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlob<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBlob(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), size, codepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFile<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadFile(&*(&pfilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcodepage, ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateReadOnlyStreamFromBlob(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDefaultIncludeHandler(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf8(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBlobAsUtf16(&*(&pblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pblobencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDxilContainerPart(&*(&pshader as *const <DxcBuffer as ::windows::core::Abi>::Abi as *const <DxcBuffer as ::windows::core::DefaultType>::DefaultType), dxcpart, ::core::mem::transmute_copy(&pppartdata), ::core::mem::transmute_copy(&ppartsizeinbytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReflection<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateReflection(&*(&pdata as *const <DxcBuffer as ::windows::core::Abi>::Abi as *const <DxcBuffer as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvreflection as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildArguments<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildArguments(
                &*(&psourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pentrypoint as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptargetprofile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                argcount,
                &*(&pdefines as *const <DxcDefine as ::windows::core::Abi>::Abi as *const <DxcDefine as ::windows::core::DefaultType>::DefaultType),
                definecount,
                ::core::mem::transmute_copy(&ppargs),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPDBContents<Impl: IDxcUtilsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdbblob: ::windows::core::RawPtr, pphash: *mut ::windows::core::RawPtr, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPDBContents(&*(&ppdbblob as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pphash), ::core::mem::transmute_copy(&ppcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDxcUtils>,
            base.5,
            CreateBlobFromBlob::<Impl, OFFSET>,
            CreateBlobFromPinned::<Impl, OFFSET>,
            MoveToBlob::<Impl, OFFSET>,
            CreateBlob::<Impl, OFFSET>,
            LoadFile::<Impl, OFFSET>,
            CreateReadOnlyStreamFromBlob::<Impl, OFFSET>,
            CreateDefaultIncludeHandler::<Impl, OFFSET>,
            GetBlobAsUtf8::<Impl, OFFSET>,
            GetBlobAsUtf16::<Impl, OFFSET>,
            GetDxilContainerPart::<Impl, OFFSET>,
            CreateReflection::<Impl, OFFSET>,
            BuildArguments::<Impl, OFFSET>,
            GetPDBContents::<Impl, OFFSET>,
        )
    }
}
pub trait IDxcValidatorImpl: Sized {
    fn Validate();
}
impl ::windows::core::RuntimeName for IDxcValidator {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcValidator";
}
impl IDxcValidatorVtbl {
    pub const fn new<Impl: IDxcValidatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcValidatorVtbl {
        unsafe extern "system" fn Validate<Impl: IDxcValidatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Validate(&*(&pshader as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcValidator>, base.5, Validate::<Impl, OFFSET>)
    }
}
pub trait IDxcValidator2Impl: Sized + IDxcValidatorImpl {
    fn ValidateWithDebug();
}
impl ::windows::core::RuntimeName for IDxcValidator2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcValidator2";
}
impl IDxcValidator2Vtbl {
    pub const fn new<Impl: IDxcValidator2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcValidator2Vtbl {
        unsafe extern "system" fn ValidateWithDebug<Impl: IDxcValidator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateWithDebug(&*(&pshader as *const <IDxcBlob as ::windows::core::Abi>::Abi as *const <IDxcBlob as ::windows::core::DefaultType>::DefaultType), flags, &*(&poptdebugbitcode as *const <DxcBuffer as ::windows::core::Abi>::Abi as *const <DxcBuffer as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcValidator2>, base.5, ValidateWithDebug::<Impl, OFFSET>)
    }
}
pub trait IDxcVersionInfoImpl: Sized {
    fn GetVersion();
    fn GetFlags();
}
impl ::windows::core::RuntimeName for IDxcVersionInfo {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcVersionInfo";
}
impl IDxcVersionInfoVtbl {
    pub const fn new<Impl: IDxcVersionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcVersionInfoVtbl {
        unsafe extern "system" fn GetVersion<Impl: IDxcVersionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pmajor), ::core::mem::transmute_copy(&pminor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IDxcVersionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcVersionInfo>, base.5, GetVersion::<Impl, OFFSET>, GetFlags::<Impl, OFFSET>)
    }
}
pub trait IDxcVersionInfo2Impl: Sized + IDxcVersionInfoImpl {
    fn GetCommitInfo();
}
impl ::windows::core::RuntimeName for IDxcVersionInfo2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcVersionInfo2";
}
impl IDxcVersionInfo2Vtbl {
    pub const fn new<Impl: IDxcVersionInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcVersionInfo2Vtbl {
        unsafe extern "system" fn GetCommitInfo<Impl: IDxcVersionInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCommitInfo(::core::mem::transmute_copy(&pcommitcount), ::core::mem::transmute_copy(&pcommithash)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcVersionInfo2>, base.5, GetCommitInfo::<Impl, OFFSET>)
    }
}
pub trait IDxcVersionInfo3Impl: Sized {
    fn GetCustomVersionString();
}
impl ::windows::core::RuntimeName for IDxcVersionInfo3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.Dxc.IDxcVersionInfo3";
}
impl IDxcVersionInfo3Vtbl {
    pub const fn new<Impl: IDxcVersionInfo3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDxcVersionInfo3Vtbl {
        unsafe extern "system" fn GetCustomVersionString<Impl: IDxcVersionInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomVersionString(::core::mem::transmute_copy(&pversionstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDxcVersionInfo3>, base.5, GetCustomVersionString::<Impl, OFFSET>)
    }
}
