#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTargetImpl: Sized {
    fn GetXpsOMPackageWriter(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(&mut self) -> ::windows::core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTargetVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter(::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *xpsfactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Impl, IMPL_OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Impl, IMPL_OFFSET>,
            GetXpsType: GetXpsType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3DImpl: Sized {
    fn GetXpsOMPackageWriter3D(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, modelpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, modeldata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(&mut self) -> ::windows::core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTarget3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTarget3DVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter3D(::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&discardcontrolpartname), ::core::mem::transmute(&modelpartname), ::core::mem::transmute(&modeldata)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *xpsfactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Impl, IMPL_OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget3D as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMBrushImpl: Sized + IXpsOMShareableImpl {
    fn GetOpacity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&mut self, opacity: f32) -> ::windows::core::Result<()>;
}
impl IXpsOMBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMBrushVtbl {
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *opacity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        Self {
            base: IXpsOMShareableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOpacity: GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMCanvasImpl: Sized + IXpsOMShareableImpl + IXpsOMVisualImpl {
    fn GetVisuals(&mut self) -> ::windows::core::Result<IXpsOMVisualCollection>;
    fn GetUseAliasedEdgeMode(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetUseAliasedEdgeMode(&mut self, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAccessibilityShortDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccessibilityShortDescription(&mut self, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccessibilityLongDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccessibilityLongDescription(&mut self, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDictionary(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&mut self, resourcedictionary: ::core::option::Option<IXpsOMDictionary>) -> ::windows::core::Result<()>;
    fn GetDictionaryResource(&mut self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&mut self, remotedictionaryresource: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMCanvas>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMCanvasVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCanvasImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCanvasVtbl {
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *visuals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseAliasedEdgeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *usealiasededgemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseAliasedEdgeMode(::core::mem::transmute_copy(&usealiasededgemode)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shortdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *longdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)).into()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryLocal(::core::mem::transmute(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryResource(::core::mem::transmute(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *canvas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMVisualVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetVisuals: GetVisuals::<Impl, IMPL_OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Impl, IMPL_OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Impl, IMPL_OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            GetDictionary: GetDictionary::<Impl, IMPL_OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Impl, IMPL_OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Impl, IMPL_OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Impl, IMPL_OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMCanvas as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByPartName(::core::mem::transmute(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            GetByPartName: GetByPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMCorePropertiesImpl: Sized + IXpsOMPartImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPackage>;
    fn GetCategory(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetCategory(&mut self, category: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetContentStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetContentStatus(&mut self, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetContentType(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetContentType(&mut self, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCreated(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetCreated(&mut self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn GetCreator(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetCreator(&mut self, creator: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDescription(&mut self, description: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetIdentifier(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetIdentifier(&mut self, identifier: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetKeywords(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetKeywords(&mut self, keywords: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetLanguage(&mut self, language: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetLastModifiedBy(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetLastModifiedBy(&mut self, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetLastPrinted(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetLastPrinted(&mut self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn GetModified(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetModified(&mut self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn GetRevision(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetRevision(&mut self, revision: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSubject(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetSubject(&mut self, subject: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetTitle(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTitle(&mut self, title: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetVersion(&mut self, version: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMCoreProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMCorePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCorePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCorePropertiesVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *category = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(::core::mem::transmute_copy(&category)).into()
        }
        unsafe extern "system" fn GetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *contentstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentStatus(::core::mem::transmute_copy(&contentstatus)).into()
        }
        unsafe extern "system" fn GetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(::core::mem::transmute_copy(&contenttype)).into()
        }
        unsafe extern "system" fn GetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *created = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreated(::core::mem::transmute_copy(&created)).into()
        }
        unsafe extern "system" fn GetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *creator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreator(::core::mem::transmute_copy(&creator)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn GetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *identifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdentifier(::core::mem::transmute_copy(&identifier)).into()
        }
        unsafe extern "system" fn GetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeywords() {
                ::core::result::Result::Ok(ok__) => {
                    *keywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeywords(::core::mem::transmute_copy(&keywords)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn GetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastModifiedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *lastmodifiedby = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastModifiedBy(::core::mem::transmute_copy(&lastmodifiedby)).into()
        }
        unsafe extern "system" fn GetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    *lastprinted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastPrinted(::core::mem::transmute_copy(&lastprinted)).into()
        }
        unsafe extern "system" fn GetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModified() {
                ::core::result::Result::Ok(ok__) => {
                    *modified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModified(::core::mem::transmute_copy(&modified)).into()
        }
        unsafe extern "system" fn GetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *revision = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevision(::core::mem::transmute_copy(&revision)).into()
        }
        unsafe extern "system" fn GetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubject() {
                ::core::result::Result::Ok(ok__) => {
                    *subject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&subject)).into()
        }
        unsafe extern "system" fn GetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&version)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *coreproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMPartVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetCategory: GetCategory::<Impl, IMPL_OFFSET>,
            SetCategory: SetCategory::<Impl, IMPL_OFFSET>,
            GetContentStatus: GetContentStatus::<Impl, IMPL_OFFSET>,
            SetContentStatus: SetContentStatus::<Impl, IMPL_OFFSET>,
            GetContentType: GetContentType::<Impl, IMPL_OFFSET>,
            SetContentType: SetContentType::<Impl, IMPL_OFFSET>,
            GetCreated: GetCreated::<Impl, IMPL_OFFSET>,
            SetCreated: SetCreated::<Impl, IMPL_OFFSET>,
            GetCreator: GetCreator::<Impl, IMPL_OFFSET>,
            SetCreator: SetCreator::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET>,
            SetIdentifier: SetIdentifier::<Impl, IMPL_OFFSET>,
            GetKeywords: GetKeywords::<Impl, IMPL_OFFSET>,
            SetKeywords: SetKeywords::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Impl, IMPL_OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Impl, IMPL_OFFSET>,
            GetLastPrinted: GetLastPrinted::<Impl, IMPL_OFFSET>,
            SetLastPrinted: SetLastPrinted::<Impl, IMPL_OFFSET>,
            GetModified: GetModified::<Impl, IMPL_OFFSET>,
            SetModified: SetModified::<Impl, IMPL_OFFSET>,
            GetRevision: GetRevision::<Impl, IMPL_OFFSET>,
            SetRevision: SetRevision::<Impl, IMPL_OFFSET>,
            GetSubject: GetSubject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            GetTitle: GetTitle::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMCoreProperties as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMDashCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<XPS_DASH>;
    fn InsertAt(&mut self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
    fn Append(&mut self, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
}
impl IXpsOMDashCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDashCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDashCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *dash = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&dash)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDashCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMDictionaryImpl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()>;
    fn GetByKey(&mut self, key: super::super::Foundation::PWSTR, beforeentry: ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<IXpsOMShareable>;
    fn GetIndex(&mut self, entry: ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<u32>;
    fn Append(&mut self, key: super::super::Foundation::PWSTR, entry: ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()>;
    fn InsertAt(&mut self, index: u32, key: super::super::Foundation::PWSTR, entry: ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, key: super::super::Foundation::PWSTR, entry: ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDictionaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDictionaryVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&entry)).into()
        }
        unsafe extern "system" fn GetByKey<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByKey(::core::mem::transmute_copy(&key), ::core::mem::transmute(&beforeentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *entry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetByKey: GetByKey::<Impl, IMPL_OFFSET>,
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDictionary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentImpl: Sized + IXpsOMPartImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocumentSequence>;
    fn GetPageReferences(&mut self) -> ::windows::core::Result<IXpsOMPageReferenceCollection>;
    fn GetPrintTicketResource(&mut self) -> ::windows::core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&mut self, printticketresource: ::core::option::Option<IXpsOMPrintTicketResource>) -> ::windows::core::Result<()>;
    fn GetDocumentStructureResource(&mut self) -> ::windows::core::Result<IXpsOMDocumentStructureResource>;
    fn SetDocumentStructureResource(&mut self, documentstructureresource: ::core::option::Option<IXpsOMDocumentStructureResource>) -> ::windows::core::Result<()>;
    fn GetSignatureBlockResources(&mut self) -> ::windows::core::Result<IXpsOMSignatureBlockResourceCollection>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageReferences() {
                ::core::result::Result::Ok(ok__) => {
                    *pagereferences = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentStructureResource() {
                ::core::result::Result::Ok(ok__) => {
                    *documentstructureresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentStructureResource(::core::mem::transmute(&documentstructureresource)).into()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlockResources() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMPartVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetPageReferences: GetPageReferences::<Impl, IMPL_OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Impl, IMPL_OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Impl, IMPL_OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Impl, IMPL_OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Impl, IMPL_OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocument as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMDocumentCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMDocument>;
    fn InsertAt(&mut self, index: u32, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
    fn Append(&mut self, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
}
impl IXpsOMDocumentCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&document)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&document)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&document)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentSequenceImpl: Sized + IXpsOMPartImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPackage>;
    fn GetDocuments(&mut self) -> ::windows::core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(&mut self) -> ::windows::core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&mut self, printticketresource: ::core::option::Option<IXpsOMPrintTicketResource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentSequenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentSequenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentSequenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *documents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        Self {
            base: IXpsOMPartVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetDocuments: GetDocuments::<Impl, IMPL_OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Impl, IMPL_OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentSequence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentStructureResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentStructureResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentStructureResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetEmbeddingOption(&mut self) -> ::windows::core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *readerstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute_copy(&embeddingoption), ::core::mem::transmute(&partname)).into()
        }
        unsafe extern "system" fn GetEmbeddingOption<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddingOption() {
                ::core::result::Result::Ok(ok__) => {
                    *embeddingoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMFontResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMFontResource>;
    fn SetAt(&mut self, index: u32, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn InsertAt(&mut self, index: u32, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByPartName(::core::mem::transmute(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            GetByPartName: GetByPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMFontResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryImpl: Sized + IXpsOMShareableImpl {
    fn GetFigures(&mut self) -> ::windows::core::Result<IXpsOMGeometryFigureCollection>;
    fn GetFillRule(&mut self) -> ::windows::core::Result<XPS_FILL_RULE>;
    fn SetFillRule(&mut self, fillrule: XPS_FILL_RULE) -> ::windows::core::Result<()>;
    fn GetTransform(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&mut self, transform: ::core::option::Option<IXpsOMMatrixTransform>) -> ::windows::core::Result<()>;
    fn GetTransformLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTransformLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryVtbl {
        unsafe extern "system" fn GetFigures<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFigures() {
                ::core::result::Result::Ok(ok__) => {
                    *figures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *fillrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(::core::mem::transmute_copy(&fillrule)).into()
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMShareableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFigures: GetFigures::<Impl, IMPL_OFFSET>,
            GetFillRule: GetFillRule::<Impl, IMPL_OFFSET>,
            SetFillRule: SetFillRule::<Impl, IMPL_OFFSET>,
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal: GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal: SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup: GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup: SetTransformLookup::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryFigureImpl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn GetSegmentData(&mut self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::Result<()>;
    fn GetSegmentTypes(&mut self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::Result<()>;
    fn GetSegmentStrokes(&mut self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSegments(&mut self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetStartPoint(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetStartPoint(&mut self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetIsClosed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsClosed(&mut self, isclosed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIsFilled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsFilled(&mut self, isfilled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSegmentCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSegmentDataCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSegmentStrokePattern(&mut self) -> ::windows::core::Result<XPS_SEGMENT_STROKE_PATTERN>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMGeometryFigure>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGeometryFigureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigureVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentData(::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&segmentdata)).into()
        }
        unsafe extern "system" fn GetSegmentTypes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentTypes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmenttypes)).into()
        }
        unsafe extern "system" fn GetSegmentStrokes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentStrokes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn SetSegments<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegments(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentdatacount), ::core::mem::transmute_copy(&segmenttypes), ::core::mem::transmute_copy(&segmentdata), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *startpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *isclosed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsClosed(::core::mem::transmute_copy(&isclosed)).into()
        }
        unsafe extern "system" fn GetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsFilled() {
                ::core::result::Result::Ok(ok__) => {
                    *isfilled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFilled(::core::mem::transmute_copy(&isfilled)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentDataCount() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentdatacount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentStrokePattern() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentstrokepattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *geometryfigure = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetSegmentData: GetSegmentData::<Impl, IMPL_OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Impl, IMPL_OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Impl, IMPL_OFFSET>,
            SetSegments: SetSegments::<Impl, IMPL_OFFSET>,
            GetStartPoint: GetStartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
            GetIsClosed: GetIsClosed::<Impl, IMPL_OFFSET>,
            SetIsClosed: SetIsClosed::<Impl, IMPL_OFFSET>,
            GetIsFilled: GetIsFilled::<Impl, IMPL_OFFSET>,
            SetIsFilled: SetIsFilled::<Impl, IMPL_OFFSET>,
            GetSegmentCount: GetSegmentCount::<Impl, IMPL_OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Impl, IMPL_OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigure as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGeometryFigureCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(&mut self, index: u32, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
    fn Append(&mut self, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
}
impl IXpsOMGeometryFigureCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigureCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *geometryfigure = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&geometryfigure)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&geometryfigure)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&geometryfigure)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigureCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMGlyphsImpl: Sized + IXpsOMShareableImpl + IXpsOMVisualImpl {
    fn GetUnicodeString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetGlyphIndexCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetGlyphIndices(&mut self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()>;
    fn GetGlyphMappingCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetGlyphMappings(&mut self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()>;
    fn GetProhibitedCaretStopCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetProhibitedCaretStops(&mut self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()>;
    fn GetBidiLevel(&mut self) -> ::windows::core::Result<u32>;
    fn GetIsSideways(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetDeviceFontName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetStyleSimulations(&mut self) -> ::windows::core::Result<XPS_STYLE_SIMULATION>;
    fn SetStyleSimulations(&mut self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::Result<()>;
    fn GetOrigin(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetOrigin(&mut self, origin: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetFontRenderingEmSize(&mut self) -> ::windows::core::Result<f32>;
    fn SetFontRenderingEmSize(&mut self, fontrenderingemsize: f32) -> ::windows::core::Result<()>;
    fn GetFontResource(&mut self) -> ::windows::core::Result<IXpsOMFontResource>;
    fn SetFontResource(&mut self, fontresource: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn GetFontFaceIndex(&mut self) -> ::windows::core::Result<i16>;
    fn SetFontFaceIndex(&mut self, fontfaceindex: i16) -> ::windows::core::Result<()>;
    fn GetFillBrush(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&mut self, fillbrush: ::core::option::Option<IXpsOMBrush>) -> ::windows::core::Result<()>;
    fn GetFillBrushLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetFillBrushLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetGlyphsEditor(&mut self) -> ::windows::core::Result<IXpsOMGlyphsEditor>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMGlyphs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMGlyphsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphsVtbl {
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    *unicodestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *indexcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *glyphmappingcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibitedcaretstopcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProhibitedCaretStops(::core::mem::transmute_copy(&prohibitedcaretstopcount), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *bidilevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    *issideways = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    *devicefontname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyleSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesimulations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyleSimulations(::core::mem::transmute_copy(&stylesimulations)).into()
        }
        unsafe extern "system" fn GetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *origin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontRenderingEmSize() {
                ::core::result::Result::Ok(ok__) => {
                    *fontrenderingemsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontRenderingEmSize(::core::mem::transmute_copy(&fontrenderingemsize)).into()
        }
        unsafe extern "system" fn GetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontResource() {
                ::core::result::Result::Ok(ok__) => {
                    *fontresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontResource(::core::mem::transmute(&fontresource)).into()
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *fontfaceindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontFaceIndex(::core::mem::transmute_copy(&fontfaceindex)).into()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *fillbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *fillbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLocal(::core::mem::transmute(&fillbrush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetGlyphsEditor<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphsEditor() {
                ::core::result::Result::Ok(ok__) => {
                    *editor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *glyphs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMVisualVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetUnicodeString: GetUnicodeString::<Impl, IMPL_OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Impl, IMPL_OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Impl, IMPL_OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Impl, IMPL_OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            GetBidiLevel: GetBidiLevel::<Impl, IMPL_OFFSET>,
            GetIsSideways: GetIsSideways::<Impl, IMPL_OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Impl, IMPL_OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Impl, IMPL_OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Impl, IMPL_OFFSET>,
            GetOrigin: GetOrigin::<Impl, IMPL_OFFSET>,
            SetOrigin: SetOrigin::<Impl, IMPL_OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Impl, IMPL_OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Impl, IMPL_OFFSET>,
            GetFontResource: GetFontResource::<Impl, IMPL_OFFSET>,
            SetFontResource: SetFontResource::<Impl, IMPL_OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Impl, IMPL_OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Impl, IMPL_OFFSET>,
            GetFillBrush: GetFillBrush::<Impl, IMPL_OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Impl, IMPL_OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Impl, IMPL_OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Impl, IMPL_OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Impl, IMPL_OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGlyphs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGlyphsEditorImpl: Sized {
    fn ApplyEdits(&mut self) -> ::windows::core::Result<()>;
    fn GetUnicodeString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetUnicodeString(&mut self, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetGlyphIndexCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetGlyphIndices(&mut self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()>;
    fn SetGlyphIndices(&mut self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::Result<()>;
    fn GetGlyphMappingCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetGlyphMappings(&mut self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()>;
    fn SetGlyphMappings(&mut self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::Result<()>;
    fn GetProhibitedCaretStopCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetProhibitedCaretStops(&mut self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()>;
    fn SetProhibitedCaretStops(&mut self, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::Result<()>;
    fn GetBidiLevel(&mut self) -> ::windows::core::Result<u32>;
    fn SetBidiLevel(&mut self, bidilevel: u32) -> ::windows::core::Result<()>;
    fn GetIsSideways(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsSideways(&mut self, issideways: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDeviceFontName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDeviceFontName(&mut self, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGlyphsEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphsEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphsEditorVtbl {
        unsafe extern "system" fn ApplyEdits<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyEdits().into()
        }
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    *unicodestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicodeString(::core::mem::transmute_copy(&unicodestring)).into()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *indexcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn SetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *glyphmappingcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn SetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibitedcaretstopcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *bidilevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBidiLevel(::core::mem::transmute_copy(&bidilevel)).into()
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    *issideways = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSideways(::core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    *devicefontname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceFontName(::core::mem::transmute_copy(&devicefontname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ApplyEdits: ApplyEdits::<Impl, IMPL_OFFSET>,
            GetUnicodeString: GetUnicodeString::<Impl, IMPL_OFFSET>,
            SetUnicodeString: SetUnicodeString::<Impl, IMPL_OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Impl, IMPL_OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Impl, IMPL_OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Impl, IMPL_OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Impl, IMPL_OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Impl, IMPL_OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            GetBidiLevel: GetBidiLevel::<Impl, IMPL_OFFSET>,
            SetBidiLevel: SetBidiLevel::<Impl, IMPL_OFFSET>,
            GetIsSideways: GetIsSideways::<Impl, IMPL_OFFSET>,
            SetIsSideways: SetIsSideways::<Impl, IMPL_OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Impl, IMPL_OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGlyphsEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGradientBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl {
    fn GetGradientStops(&mut self) -> ::windows::core::Result<IXpsOMGradientStopCollection>;
    fn GetTransform(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&mut self, transform: ::core::option::Option<IXpsOMMatrixTransform>) -> ::windows::core::Result<()>;
    fn GetTransformLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTransformLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSpreadMethod(&mut self) -> ::windows::core::Result<XPS_SPREAD_METHOD>;
    fn SetSpreadMethod(&mut self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()>;
    fn GetColorInterpolationMode(&mut self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION>;
    fn SetColorInterpolationMode(&mut self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientBrushVtbl {
        unsafe extern "system" fn GetGradientStops<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGradientStops() {
                ::core::result::Result::Ok(ok__) => {
                    *gradientstops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpreadMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *spreadmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpreadMethod(::core::mem::transmute_copy(&spreadmethod)).into()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *colorinterpolationmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorInterpolationMode(::core::mem::transmute_copy(&colorinterpolationmode)).into()
        }
        Self {
            base: IXpsOMBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGradientStops: GetGradientStops::<Impl, IMPL_OFFSET>,
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal: GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal: SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup: GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup: SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Impl, IMPL_OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Impl, IMPL_OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGradientStopImpl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMGradientBrush>;
    fn GetOffset(&mut self) -> ::windows::core::Result<f32>;
    fn SetOffset(&mut self, offset: f32) -> ::windows::core::Result<()>;
    fn GetColor(&mut self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn SetColor(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMGradientStop>;
}
impl IXpsOMGradientStopVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStopImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStopVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *offset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *gradientstop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetOffset: GetOffset::<Impl, IMPL_OFFSET>,
            SetOffset: SetOffset::<Impl, IMPL_OFFSET>,
            GetColor: GetColor::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientStop as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGradientStopCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMGradientStop>;
    fn InsertAt(&mut self, index: u32, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
    fn Append(&mut self, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
}
impl IXpsOMGradientStopCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStopCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStopCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *stop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&stop)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&stop)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&stop)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientStopCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMImageBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl + IXpsOMTileBrushImpl {
    fn GetImageResource(&mut self) -> ::windows::core::Result<IXpsOMImageResource>;
    fn SetImageResource(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn GetColorProfileResource(&mut self) -> ::windows::core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(&mut self, colorprofileresource: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMImageBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMImageBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageBrushVtbl {
        unsafe extern "system" fn GetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn GetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResource() {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorProfileResource(::core::mem::transmute(&colorprofileresource)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *imagebrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMTileBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetImageResource: GetImageResource::<Impl, IMPL_OFFSET>,
            SetImageResource: SetImageResource::<Impl, IMPL_OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Impl, IMPL_OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetImageType(&mut self) -> ::windows::core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *readerstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute_copy(&imagetype), ::core::mem::transmute(&partname)).into()
        }
        unsafe extern "system" fn GetImageType<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageType() {
                ::core::result::Result::Ok(ok__) => {
                    *imagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            GetImageType: GetImageType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMImageResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByPartName(::core::mem::transmute(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *part = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            GetByPartName: GetByPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMLinearGradientBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl + IXpsOMGradientBrushImpl {
    fn GetStartPoint(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetStartPoint(&mut self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetEndPoint(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetEndPoint(&mut self, endpoint: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMLinearGradientBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMLinearGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMLinearGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMLinearGradientBrushVtbl {
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *startpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *endpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(::core::mem::transmute_copy(&endpoint)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *lineargradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMGradientBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStartPoint: GetStartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint: SetStartPoint::<Impl, IMPL_OFFSET>,
            GetEndPoint: GetEndPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint: SetEndPoint::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMLinearGradientBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMMatrixTransformImpl: Sized + IXpsOMShareableImpl {
    fn GetMatrix(&mut self) -> ::windows::core::Result<XPS_MATRIX>;
    fn SetMatrix(&mut self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
}
impl IXpsOMMatrixTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMMatrixTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMMatrixTransformVtbl {
        unsafe extern "system" fn GetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *matrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMShareableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMatrix: GetMatrix::<Impl, IMPL_OFFSET>,
            SetMatrix: SetMatrix::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMMatrixTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMNameCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMNameCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMNameCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMNameCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMNameCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMNameCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMNameCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactoryImpl: Sized {
    fn CreatePackage(&mut self) -> ::windows::core::Result<IXpsOMPackage>;
    fn CreatePackageFromFile(&mut self, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPackage>;
    fn CreatePackageFromStream(&mut self, stream: ::core::option::Option<super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPackage>;
    fn CreateStoryFragmentsResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMStoryFragmentsResource>;
    fn CreateDocumentStructureResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMDocumentStructureResource>;
    fn CreateSignatureBlockResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMSignatureBlockResource>;
    fn CreateRemoteDictionaryResource(&mut self, dictionary: ::core::option::Option<IXpsOMDictionary>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreateRemoteDictionaryResourceFromStream(&mut self, dictionarymarkupstream: ::core::option::Option<super::super::System::Com::IStream>, dictionaryparturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<IXpsOMPartResources>) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreatePartResources(&mut self) -> ::windows::core::Result<IXpsOMPartResources>;
    fn CreateDocumentSequence(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMDocumentSequence>;
    fn CreateDocument(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMDocument>;
    fn CreatePageReference(&mut self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference>;
    fn CreatePage(&mut self, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPage>;
    fn CreatePageFromStream(&mut self, pagemarkupstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPage>;
    fn CreateCanvas(&mut self) -> ::windows::core::Result<IXpsOMCanvas>;
    fn CreateGlyphs(&mut self, fontresource: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<IXpsOMGlyphs>;
    fn CreatePath(&mut self) -> ::windows::core::Result<IXpsOMPath>;
    fn CreateGeometry(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn CreateGeometryFigure(&mut self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure>;
    fn CreateMatrixTransform(&mut self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn CreateSolidColorBrush(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<IXpsOMSolidColorBrush>;
    fn CreateColorProfileResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMColorProfileResource>;
    fn CreateImageBrush(&mut self, image: ::core::option::Option<IXpsOMImageResource>, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush>;
    fn CreateVisualBrush(&mut self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush>;
    fn CreateImageResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, contenttype: XPS_IMAGE_TYPE, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMImageResource>;
    fn CreatePrintTicketResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPrintTicketResource>;
    fn CreateFontResource(&mut self, acquiredstream: ::core::option::Option<super::super::System::Com::IStream>, fontembedding: XPS_FONT_EMBEDDING, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, isobfsourcestream: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMFontResource>;
    fn CreateGradientStop(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop>;
    fn CreateLinearGradientBrush(&mut self, gradstop1: ::core::option::Option<IXpsOMGradientStop>, gradstop2: ::core::option::Option<IXpsOMGradientStop>, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush>;
    fn CreateRadialGradientBrush(&mut self, gradstop1: ::core::option::Option<IXpsOMGradientStop>, gradstop2: ::core::option::Option<IXpsOMGradientStop>, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush>;
    fn CreateCoreProperties(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMCoreProperties>;
    fn CreateDictionary(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn CreatePartUriCollection(&mut self) -> ::windows::core::Result<IXpsOMPartUriCollection>;
    fn CreatePackageWriterOnFile(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream(&mut self, outputstream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn CreatePartUri(&mut self, uri: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateReadOnlyStreamOnFile(&mut self, filename: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactoryVtbl {
        unsafe extern "system" fn CreatePackage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoryFragmentsResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyfragmentsresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentStructureResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *documentstructureresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSignatureBlockResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResource(::core::mem::transmute(&dictionary), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResourceFromStream(::core::mem::transmute(&dictionarymarkupstream), ::core::mem::transmute(&dictionaryparturi), ::core::mem::transmute(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *dictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartResources() {
                ::core::result::Result::Ok(ok__) => {
                    *partresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentSequence(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocument(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageReference(::core::mem::transmute_copy(&advisorypagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePage(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute_copy(&language), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream(::core::mem::transmute(&pagemarkupstream), ::core::mem::transmute(&parturi), ::core::mem::transmute(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCanvas() {
                ::core::result::Result::Ok(ok__) => {
                    *canvas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGlyphs(::core::mem::transmute(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *glyphs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryFigure(::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *figure = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform(::core::mem::transmute_copy(&matrix)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSolidColorBrush(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *solidcolorbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorProfileResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImageBrush(::core::mem::transmute(&image), ::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *imagebrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisualBrush(::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *visualbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImageResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute_copy(&contenttype), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrintTicketResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute_copy(&fontembedding), ::core::mem::transmute(&parturi), ::core::mem::transmute_copy(&isobfsourcestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *fontresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGradientStop(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *gradientstop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearGradientBrush(::core::mem::transmute(&gradstop1), ::core::mem::transmute(&gradstop2), ::core::mem::transmute_copy(&startpoint), ::core::mem::transmute_copy(&endpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *lineargradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRadialGradientBrush(::core::mem::transmute(&gradstop1), ::core::mem::transmute(&gradstop2), ::core::mem::transmute_copy(&centerpoint), ::core::mem::transmute_copy(&gradientorigin), ::core::mem::transmute_copy(&radiisizes)) {
                ::core::result::Result::Ok(ok__) => {
                    *radialgradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCoreProperties(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *coreproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartUriCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *parturicollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartUri(::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReadOnlyStreamOnFile(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePackage: CreatePackage::<Impl, IMPL_OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Impl, IMPL_OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Impl, IMPL_OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Impl, IMPL_OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Impl, IMPL_OFFSET>,
            CreatePartResources: CreatePartResources::<Impl, IMPL_OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Impl, IMPL_OFFSET>,
            CreateDocument: CreateDocument::<Impl, IMPL_OFFSET>,
            CreatePageReference: CreatePageReference::<Impl, IMPL_OFFSET>,
            CreatePage: CreatePage::<Impl, IMPL_OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Impl, IMPL_OFFSET>,
            CreateCanvas: CreateCanvas::<Impl, IMPL_OFFSET>,
            CreateGlyphs: CreateGlyphs::<Impl, IMPL_OFFSET>,
            CreatePath: CreatePath::<Impl, IMPL_OFFSET>,
            CreateGeometry: CreateGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Impl, IMPL_OFFSET>,
            CreateImageBrush: CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Impl, IMPL_OFFSET>,
            CreateImageResource: CreateImageResource::<Impl, IMPL_OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Impl, IMPL_OFFSET>,
            CreateFontResource: CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateGradientStop: CreateGradientStop::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Impl, IMPL_OFFSET>,
            CreateDictionary: CreateDictionary::<Impl, IMPL_OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Impl, IMPL_OFFSET>,
            CreatePartUri: CreatePartUri::<Impl, IMPL_OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1Impl: Sized + IXpsOMObjectFactoryImpl {
    fn GetDocumentTypeFromFile(&mut self, filename: super::super::Foundation::PWSTR) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn GetDocumentTypeFromStream(&mut self, xpsdocumentstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn ConvertHDPhotoToJpegXR(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn ConvertJpegXRToHDPhoto(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn CreatePackageWriterOnFile1(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream1(&mut self, outputstream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn CreatePackage1(&mut self) -> ::windows::core::Result<IXpsOMPackage1>;
    fn CreatePackageFromStream1(&mut self, stream: ::core::option::Option<super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPackage1>;
    fn CreatePackageFromFile1(&mut self, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPackage1>;
    fn CreatePage1(&mut self, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPage1>;
    fn CreatePageFromStream1(&mut self, pagemarkupstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows::core::Result<IXpsOMPage1>;
    fn CreateRemoteDictionaryResourceFromStream1(&mut self, dictionarymarkupstream: ::core::option::Option<super::super::System::Com::IStream>, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<IXpsOMPartResources>) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactory1Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromFile(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromStream(::core::mem::transmute(&xpsdocumentstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertHDPhotoToJpegXR(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertJpegXRToHDPhoto(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnFile1(
                ::core::mem::transmute_copy(&filename),
                ::core::mem::transmute_copy(&securityattributes),
                ::core::mem::transmute_copy(&flagsandattributes),
                ::core::mem::transmute_copy(&optimizemarkupsize),
                ::core::mem::transmute_copy(&interleaving),
                ::core::mem::transmute(&documentsequencepartname),
                ::core::mem::transmute(&coreproperties),
                ::core::mem::transmute(&packagethumbnail),
                ::core::mem::transmute(&documentsequenceprintticket),
                ::core::mem::transmute(&discardcontrolpartname),
                ::core::mem::transmute_copy(&documenttype),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream1(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname), ::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackage1() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile1(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePage1(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute_copy(&language), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream1(::core::mem::transmute(&pagemarkupstream), ::core::mem::transmute(&parturi), ::core::mem::transmute(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResourceFromStream1(::core::mem::transmute(&dictionarymarkupstream), ::core::mem::transmute(&parturi), ::core::mem::transmute(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *dictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMObjectFactoryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Impl, IMPL_OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Impl, IMPL_OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Impl, IMPL_OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Impl, IMPL_OFFSET>,
            CreatePackage1: CreatePackage1::<Impl, IMPL_OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Impl, IMPL_OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Impl, IMPL_OFFSET>,
            CreatePage1: CreatePage1::<Impl, IMPL_OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageImpl: Sized {
    fn GetDocumentSequence(&mut self) -> ::windows::core::Result<IXpsOMDocumentSequence>;
    fn SetDocumentSequence(&mut self, documentsequence: ::core::option::Option<IXpsOMDocumentSequence>) -> ::windows::core::Result<()>;
    fn GetCoreProperties(&mut self) -> ::windows::core::Result<IXpsOMCoreProperties>;
    fn SetCoreProperties(&mut self, coreproperties: ::core::option::Option<IXpsOMCoreProperties>) -> ::windows::core::Result<()>;
    fn GetDiscardControlPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetDiscardControlPartName(&mut self, discardcontrolparturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetThumbnailResource(&mut self) -> ::windows::core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn WriteToFile(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn WriteToStream(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageVtbl {
        unsafe extern "system" fn GetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentSequence(::core::mem::transmute(&documentsequence)).into()
        }
        unsafe extern "system" fn GetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCoreProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *coreproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoreProperties(::core::mem::transmute(&coreproperties)).into()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiscardControlPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *discardcontrolparturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscardControlPartName(::core::mem::transmute(&discardcontrolparturi)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn WriteToFile<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn WriteToStream<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDocumentSequence: GetDocumentSequence::<Impl, IMPL_OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Impl, IMPL_OFFSET>,
            GetCoreProperties: GetCoreProperties::<Impl, IMPL_OFFSET>,
            SetCoreProperties: SetCoreProperties::<Impl, IMPL_OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Impl, IMPL_OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Impl, IMPL_OFFSET>,
            WriteToFile: WriteToFile::<Impl, IMPL_OFFSET>,
            WriteToStream: WriteToStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1Impl: Sized + IXpsOMPackageImpl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
    fn WriteToStream1(&mut self, outputstream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackage1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToFile1(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        unsafe extern "system" fn WriteToStream1<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToStream1(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMPackageVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentType: GetDocumentType::<Impl, IMPL_OFFSET>,
            WriteToFile1: WriteToFile1::<Impl, IMPL_OFFSET>,
            WriteToStream1: WriteToStream1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackage1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageTargetImpl: Sized {
    fn CreateXpsOMPackageWriter(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageTargetVtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Impl: IXpsOMPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateXpsOMPackageWriter(::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriterImpl: Sized {
    fn StartNewDocument(&mut self, documentpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documentprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, documentstructure: ::core::option::Option<IXpsOMDocumentStructureResource>, signatureblockresources: ::core::option::Option<IXpsOMSignatureBlockResourceCollection>, restrictedfonts: ::core::option::Option<IXpsOMPartUriCollection>) -> ::windows::core::Result<()>;
    fn AddPage(&mut self, page: ::core::option::Option<IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::core::option::Option<IXpsOMPartUriCollection>, storyfragments: ::core::option::Option<IXpsOMStoryFragmentsResource>, pageprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, pagethumbnail: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn AddResource(&mut self, resource: ::core::option::Option<IXpsOMResource>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsClosed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriterVtbl {
        unsafe extern "system" fn StartNewDocument<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNewDocument(::core::mem::transmute(&documentpartname), ::core::mem::transmute(&documentprintticket), ::core::mem::transmute(&documentstructure), ::core::mem::transmute(&signatureblockresources), ::core::mem::transmute(&restrictedfonts)).into()
        }
        unsafe extern "system" fn AddPage<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPage(::core::mem::transmute(&page), ::core::mem::transmute_copy(&advisorypagedimensions), ::core::mem::transmute(&discardableresourceparts), ::core::mem::transmute(&storyfragments), ::core::mem::transmute(&pageprintticket), ::core::mem::transmute(&pagethumbnail)).into()
        }
        unsafe extern "system" fn AddResource<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResource(::core::mem::transmute(&resource)).into()
        }
        unsafe extern "system" fn Close<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsClosed<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *isclosed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartNewDocument: StartNewDocument::<Impl, IMPL_OFFSET>,
            AddPage: AddPage::<Impl, IMPL_OFFSET>,
            AddResource: AddResource::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            IsClosed: IsClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3DImpl: Sized + IXpsOMPackageWriterImpl {
    fn AddModelTexture(&mut self, texturepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, texturedata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetModelPrintTicket(&mut self, printticketpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, printticketdata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriter3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriter3DVtbl {
        unsafe extern "system" fn AddModelTexture<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddModelTexture(::core::mem::transmute(&texturepartname), ::core::mem::transmute(&texturedata)).into()
        }
        unsafe extern "system" fn SetModelPrintTicket<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelPrintTicket(::core::mem::transmute(&printticketpartname), ::core::mem::transmute(&printticketdata)).into()
        }
        Self {
            base: IXpsOMPackageWriterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddModelTexture: AddModelTexture::<Impl, IMPL_OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPageImpl: Sized + IXpsOMPartImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPageReference>;
    fn GetVisuals(&mut self) -> ::windows::core::Result<IXpsOMVisualCollection>;
    fn GetPageDimensions(&mut self) -> ::windows::core::Result<XPS_SIZE>;
    fn SetPageDimensions(&mut self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()>;
    fn GetContentBox(&mut self) -> ::windows::core::Result<XPS_RECT>;
    fn SetContentBox(&mut self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()>;
    fn GetBleedBox(&mut self) -> ::windows::core::Result<XPS_RECT>;
    fn SetBleedBox(&mut self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()>;
    fn GetLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetLanguage(&mut self, language: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetName(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetIsHyperlinkTarget(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&mut self, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDictionary(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&mut self, resourcedictionary: ::core::option::Option<IXpsOMDictionary>) -> ::windows::core::Result<()>;
    fn GetDictionaryResource(&mut self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&mut self, remotedictionaryresource: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn Write(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GenerateUnusedLookupKey(&mut self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMPage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *visuals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pagedimensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetContentBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentBox() {
                ::core::result::Result::Ok(ok__) => {
                    *contentbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentBox(::core::mem::transmute_copy(&contentbox)).into()
        }
        unsafe extern "system" fn GetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBleedBox() {
                ::core::result::Result::Ok(ok__) => {
                    *bleedbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBleedBox(::core::mem::transmute_copy(&bleedbox)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ishyperlinktarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlinktarget)).into()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryLocal(::core::mem::transmute(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryResource(::core::mem::transmute(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Write<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateUnusedLookupKey(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMPartVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetVisuals: GetVisuals::<Impl, IMPL_OFFSET>,
            GetPageDimensions: GetPageDimensions::<Impl, IMPL_OFFSET>,
            SetPageDimensions: SetPageDimensions::<Impl, IMPL_OFFSET>,
            GetContentBox: GetContentBox::<Impl, IMPL_OFFSET>,
            SetContentBox: SetContentBox::<Impl, IMPL_OFFSET>,
            GetBleedBox: GetBleedBox::<Impl, IMPL_OFFSET>,
            SetBleedBox: SetBleedBox::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetDictionary: GetDictionary::<Impl, IMPL_OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Impl, IMPL_OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Impl, IMPL_OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Impl, IMPL_OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1Impl: Sized + IXpsOMPartImpl + IXpsOMPageImpl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPage1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMPage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMPageVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentType: GetDocumentType::<Impl, IMPL_OFFSET>,
            Write1: Write1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPage1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMPageReferenceImpl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
    fn GetPage(&mut self) -> ::windows::core::Result<IXpsOMPage>;
    fn SetPage(&mut self, page: ::core::option::Option<IXpsOMPage>) -> ::windows::core::Result<()>;
    fn DiscardPage(&mut self) -> ::windows::core::Result<()>;
    fn IsPageLoaded(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetAdvisoryPageDimensions(&mut self) -> ::windows::core::Result<XPS_SIZE>;
    fn SetAdvisoryPageDimensions(&mut self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()>;
    fn GetStoryFragmentsResource(&mut self) -> ::windows::core::Result<IXpsOMStoryFragmentsResource>;
    fn SetStoryFragmentsResource(&mut self, storyfragmentsresource: ::core::option::Option<IXpsOMStoryFragmentsResource>) -> ::windows::core::Result<()>;
    fn GetPrintTicketResource(&mut self) -> ::windows::core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&mut self, printticketresource: ::core::option::Option<IXpsOMPrintTicketResource>) -> ::windows::core::Result<()>;
    fn GetThumbnailResource(&mut self) -> ::windows::core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn CollectLinkTargets(&mut self) -> ::windows::core::Result<IXpsOMNameCollection>;
    fn CollectPartResources(&mut self) -> ::windows::core::Result<IXpsOMPartResources>;
    fn HasRestrictedFonts(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMPageReference>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMPageReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReferenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPage() {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPage(::core::mem::transmute(&page)).into()
        }
        unsafe extern "system" fn DiscardPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardPage().into()
        }
        unsafe extern "system" fn IsPageLoaded<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPageLoaded() {
                ::core::result::Result::Ok(ok__) => {
                    *ispageloaded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdvisoryPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pagedimensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvisoryPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryFragmentsResource() {
                ::core::result::Result::Ok(ok__) => {
                    *storyfragmentsresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryFragmentsResource(::core::mem::transmute(&storyfragmentsresource)).into()
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn CollectLinkTargets<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectLinkTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *linktargets = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectPartResources() {
                ::core::result::Result::Ok(ok__) => {
                    *partresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasRestrictedFonts() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictedfonts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetPage: GetPage::<Impl, IMPL_OFFSET>,
            SetPage: SetPage::<Impl, IMPL_OFFSET>,
            DiscardPage: DiscardPage::<Impl, IMPL_OFFSET>,
            IsPageLoaded: IsPageLoaded::<Impl, IMPL_OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Impl, IMPL_OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Impl, IMPL_OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Impl, IMPL_OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Impl, IMPL_OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Impl, IMPL_OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Impl, IMPL_OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Impl, IMPL_OFFSET>,
            CollectPartResources: CollectPartResources::<Impl, IMPL_OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPageReference as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMPageReferenceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMPageReference>;
    fn InsertAt(&mut self, index: u32, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
    fn Append(&mut self, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
}
impl IXpsOMPageReferenceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReferenceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReferenceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&pagereference)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&pagereference)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&pagereference)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPageReferenceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartImpl: Sized {
    fn GetPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartVtbl {
        unsafe extern "system" fn GetPartName<Impl: IXpsOMPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Impl: IXpsOMPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPartName(::core::mem::transmute(&parturi)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPartName: GetPartName::<Impl, IMPL_OFFSET>,
            SetPartName: SetPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPart as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMPartResourcesImpl: Sized {
    fn GetFontResources(&mut self) -> ::windows::core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(&mut self) -> ::windows::core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(&mut self) -> ::windows::core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(&mut self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl IXpsOMPartResourcesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartResourcesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartResourcesVtbl {
        unsafe extern "system" fn GetFontResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontResources() {
                ::core::result::Result::Ok(ok__) => {
                    *fontresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageResources() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResources() {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteDictionaryResources() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionaryresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFontResources: GetFontResources::<Impl, IMPL_OFFSET>,
            GetImageResources: GetImageResources::<Impl, IMPL_OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Impl, IMPL_OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPartResources as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartUriCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(&mut self, index: u32, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn Append(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartUriCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartUriCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartUriCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&parturi)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&parturi)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&parturi)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPartUriCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMPathImpl: Sized + IXpsOMShareableImpl + IXpsOMVisualImpl {
    fn GetGeometry(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn GetGeometryLocal(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn SetGeometryLocal(&mut self, geometry: ::core::option::Option<IXpsOMGeometry>) -> ::windows::core::Result<()>;
    fn GetGeometryLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetGeometryLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccessibilityShortDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccessibilityShortDescription(&mut self, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccessibilityLongDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetAccessibilityLongDescription(&mut self, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSnapsToPixels(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetSnapsToPixels(&mut self, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetStrokeBrush(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn GetStrokeBrushLocal(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn SetStrokeBrushLocal(&mut self, brush: ::core::option::Option<IXpsOMBrush>) -> ::windows::core::Result<()>;
    fn GetStrokeBrushLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetStrokeBrushLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStrokeDashes(&mut self) -> ::windows::core::Result<IXpsOMDashCollection>;
    fn GetStrokeDashCap(&mut self) -> ::windows::core::Result<XPS_DASH_CAP>;
    fn SetStrokeDashCap(&mut self, strokedashcap: XPS_DASH_CAP) -> ::windows::core::Result<()>;
    fn GetStrokeDashOffset(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeDashOffset(&mut self, strokedashoffset: f32) -> ::windows::core::Result<()>;
    fn GetStrokeStartLineCap(&mut self) -> ::windows::core::Result<XPS_LINE_CAP>;
    fn SetStrokeStartLineCap(&mut self, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()>;
    fn GetStrokeEndLineCap(&mut self) -> ::windows::core::Result<XPS_LINE_CAP>;
    fn SetStrokeEndLineCap(&mut self, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()>;
    fn GetStrokeLineJoin(&mut self) -> ::windows::core::Result<XPS_LINE_JOIN>;
    fn SetStrokeLineJoin(&mut self, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::Result<()>;
    fn GetStrokeMiterLimit(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeMiterLimit(&mut self, strokemiterlimit: f32) -> ::windows::core::Result<()>;
    fn GetStrokeThickness(&mut self) -> ::windows::core::Result<f32>;
    fn SetStrokeThickness(&mut self, strokethickness: f32) -> ::windows::core::Result<()>;
    fn GetFillBrush(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&mut self, brush: ::core::option::Option<IXpsOMBrush>) -> ::windows::core::Result<()>;
    fn GetFillBrushLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetFillBrushLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMPath>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPathVtbl {
        unsafe extern "system" fn GetGeometry<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometryLocal(::core::mem::transmute(&geometry)).into()
        }
        unsafe extern "system" fn GetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometryLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shortdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *longdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)).into()
        }
        unsafe extern "system" fn GetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapsToPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *snapstopixels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapsToPixels(::core::mem::transmute_copy(&snapstopixels)).into()
        }
        unsafe extern "system" fn GetStrokeBrush<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrushLocal(::core::mem::transmute(&brush)).into()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrushLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn GetStrokeDashes<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashcap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashCap(::core::mem::transmute_copy(&strokedashcap)).into()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashOffset(::core::mem::transmute_copy(&strokedashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeStartLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokestartlinecap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartLineCap(::core::mem::transmute_copy(&strokestartlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeEndLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokeendlinecap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeEndLineCap(::core::mem::transmute_copy(&strokeendlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    *strokelinejoin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeLineJoin(::core::mem::transmute_copy(&strokelinejoin)).into()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *strokemiterlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeMiterLimit(::core::mem::transmute_copy(&strokemiterlimit)).into()
        }
        unsafe extern "system" fn GetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *strokethickness = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(::core::mem::transmute_copy(&strokethickness)).into()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLocal(::core::mem::transmute(&brush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMVisualVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGeometry: GetGeometry::<Impl, IMPL_OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Impl, IMPL_OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Impl, IMPL_OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Impl, IMPL_OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Impl, IMPL_OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Impl, IMPL_OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Impl, IMPL_OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Impl, IMPL_OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Impl, IMPL_OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Impl, IMPL_OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Impl, IMPL_OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Impl, IMPL_OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Impl, IMPL_OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Impl, IMPL_OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Impl, IMPL_OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Impl, IMPL_OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Impl, IMPL_OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Impl, IMPL_OFFSET>,
            GetFillBrush: GetFillBrush::<Impl, IMPL_OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Impl, IMPL_OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Impl, IMPL_OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Impl, IMPL_OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPrintTicketResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPrintTicketResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMRadialGradientBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl + IXpsOMGradientBrushImpl {
    fn GetCenter(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetCenter(&mut self, center: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetRadiiSizes(&mut self) -> ::windows::core::Result<XPS_SIZE>;
    fn SetRadiiSizes(&mut self, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<()>;
    fn GetGradientOrigin(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetGradientOrigin(&mut self, origin: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMRadialGradientBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMRadialGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRadialGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRadialGradientBrushVtbl {
        unsafe extern "system" fn GetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *center = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(::core::mem::transmute_copy(&center)).into()
        }
        unsafe extern "system" fn GetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadiiSizes() {
                ::core::result::Result::Ok(ok__) => {
                    *radiisizes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiiSizes(::core::mem::transmute_copy(&radiisizes)).into()
        }
        unsafe extern "system" fn GetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGradientOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *origin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGradientOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *radialgradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMGradientBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCenter: GetCenter::<Impl, IMPL_OFFSET>,
            SetCenter: SetCenter::<Impl, IMPL_OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Impl, IMPL_OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Impl, IMPL_OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Impl, IMPL_OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRadialGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetDictionary(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn SetDictionary(&mut self, dictionary: ::core::option::Option<IXpsOMDictionary>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceVtbl {
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionary(::core::mem::transmute(&dictionary)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDictionary: GetDictionary::<Impl, IMPL_OFFSET>,
            SetDictionary: SetDictionary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1Impl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl + IXpsOMRemoteDictionaryResourceImpl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResource1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMRemoteDictionaryResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentType: GetDocumentType::<Impl, IMPL_OFFSET>,
            Write1: Write1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByPartName(::core::mem::transmute(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            GetByPartName: GetByPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMResourceImpl: Sized + IXpsOMPartImpl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMResourceVtbl {
        Self { base: IXpsOMPartVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMResource as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMShareableImpl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetType(&mut self) -> ::windows::core::Result<XPS_OBJECT_TYPE>;
}
impl IXpsOMShareableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMShareableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMShareableVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMShareableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IXpsOMShareableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMShareable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(&mut self, index: u32, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByPartName(::core::mem::transmute(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            GetByPartName: GetByPartName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResourceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMSolidColorBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl {
    fn GetColor(&mut self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn SetColor(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMSolidColorBrush>;
}
impl IXpsOMSolidColorBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSolidColorBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSolidColorBrushVtbl {
        unsafe extern "system" fn GetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *solidcolorbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetColor: GetColor::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSolidColorBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResourceImpl: Sized + IXpsOMPartImpl + IXpsOMResourceImpl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPageReference>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMStoryFragmentsResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMStoryFragmentsResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMStoryFragmentsResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMThumbnailGeneratorImpl: Sized {
    fn GenerateThumbnail(&mut self, page: ::core::option::Option<IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMThumbnailGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMThumbnailGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMThumbnailGeneratorVtbl {
        unsafe extern "system" fn GenerateThumbnail<Impl: IXpsOMThumbnailGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateThumbnail(::core::mem::transmute(&page), ::core::mem::transmute_copy(&thumbnailtype), ::core::mem::transmute_copy(&thumbnailsize), ::core::mem::transmute(&imageresourcepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GenerateThumbnail: GenerateThumbnail::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMThumbnailGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMTileBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl {
    fn GetTransform(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&mut self, transform: ::core::option::Option<IXpsOMMatrixTransform>) -> ::windows::core::Result<()>;
    fn GetTransformLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTransformLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetViewbox(&mut self) -> ::windows::core::Result<XPS_RECT>;
    fn SetViewbox(&mut self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()>;
    fn GetViewport(&mut self) -> ::windows::core::Result<XPS_RECT>;
    fn SetViewport(&mut self, viewport: *const XPS_RECT) -> ::windows::core::Result<()>;
    fn GetTileMode(&mut self) -> ::windows::core::Result<XPS_TILE_MODE>;
    fn SetTileMode(&mut self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMTileBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMTileBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMTileBrushVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewbox() {
                ::core::result::Result::Ok(ok__) => {
                    *viewbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewbox(::core::mem::transmute_copy(&viewbox)).into()
        }
        unsafe extern "system" fn GetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *viewport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewport(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn GetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTileMode() {
                ::core::result::Result::Ok(ok__) => {
                    *tilemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileMode(::core::mem::transmute_copy(&tilemode)).into()
        }
        Self {
            base: IXpsOMBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal: GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal: SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup: GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup: SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetViewbox: GetViewbox::<Impl, IMPL_OFFSET>,
            SetViewbox: SetViewbox::<Impl, IMPL_OFFSET>,
            GetViewport: GetViewport::<Impl, IMPL_OFFSET>,
            SetViewport: SetViewport::<Impl, IMPL_OFFSET>,
            GetTileMode: GetTileMode::<Impl, IMPL_OFFSET>,
            SetTileMode: SetTileMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMTileBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMVisualImpl: Sized + IXpsOMShareableImpl {
    fn GetTransform(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&mut self, matrixtransform: ::core::option::Option<IXpsOMMatrixTransform>) -> ::windows::core::Result<()>;
    fn GetTransformLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTransformLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetClipGeometry(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn GetClipGeometryLocal(&mut self) -> ::windows::core::Result<IXpsOMGeometry>;
    fn SetClipGeometryLocal(&mut self, clipgeometry: ::core::option::Option<IXpsOMGeometry>) -> ::windows::core::Result<()>;
    fn GetClipGeometryLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetClipGeometryLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetOpacity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&mut self, opacity: f32) -> ::windows::core::Result<()>;
    fn GetOpacityMaskBrush(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn GetOpacityMaskBrushLocal(&mut self) -> ::windows::core::Result<IXpsOMBrush>;
    fn SetOpacityMaskBrushLocal(&mut self, opacitymaskbrush: ::core::option::Option<IXpsOMBrush>) -> ::windows::core::Result<()>;
    fn GetOpacityMaskBrushLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetOpacityMaskBrushLookup(&mut self, key: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetName(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetIsHyperlinkTarget(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&mut self, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetHyperlinkNavigateUri(&mut self) -> ::windows::core::Result<super::super::System::Com::IUri>;
    fn SetHyperlinkNavigateUri(&mut self, hyperlinkuri: ::core::option::Option<super::super::System::Com::IUri>) -> ::windows::core::Result<()>;
    fn GetLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetLanguage(&mut self, language: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&matrixtransform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetClipGeometry<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *clipgeometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *clipgeometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipGeometryLocal(::core::mem::transmute(&clipgeometry)).into()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipGeometryLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *opacity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *opacitymaskbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *opacitymaskbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMaskBrushLocal(::core::mem::transmute(&opacitymaskbrush)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMaskBrushLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ishyperlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlink)).into()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHyperlinkNavigateUri() {
                ::core::result::Result::Ok(ok__) => {
                    *hyperlinkuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHyperlinkNavigateUri(::core::mem::transmute(&hyperlinkuri)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        Self {
            base: IXpsOMShareableVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTransform: GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal: GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal: SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup: GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup: SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetClipGeometry: GetClipGeometry::<Impl, IMPL_OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            GetOpacity: GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMVisualBrushImpl: Sized + IXpsOMShareableImpl + IXpsOMBrushImpl + IXpsOMTileBrushImpl {
    fn GetVisual(&mut self) -> ::windows::core::Result<IXpsOMVisual>;
    fn GetVisualLocal(&mut self) -> ::windows::core::Result<IXpsOMVisual>;
    fn SetVisualLocal(&mut self, visual: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn GetVisualLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetVisualLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMVisualBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMVisualBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualBrushVtbl {
        unsafe extern "system" fn GetVisual<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualLocal(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn GetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *visualbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXpsOMTileBrushVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetVisual: GetVisual::<Impl, IMPL_OFFSET>,
            GetVisualLocal: GetVisualLocal::<Impl, IMPL_OFFSET>,
            SetVisualLocal: SetVisualLocal::<Impl, IMPL_OFFSET>,
            GetVisualLookup: GetVisualLookup::<Impl, IMPL_OFFSET>,
            SetVisualLookup: SetVisualLookup::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisualBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMVisualCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMVisual>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
}
impl IXpsOMVisualCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            SetAt: SetAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisualCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureImpl: Sized {
    fn GetSignatureId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSignatureValue(&mut self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn GetCertificateEnumerator(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateEnumerator>;
    fn GetSigningTime(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSigningTimeFormat(&mut self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn GetSignaturePartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn Verify(&mut self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<XPS_SIGNATURE_STATUS>;
    fn GetPolicy(&mut self) -> ::windows::core::Result<XPS_SIGN_POLICY>;
    fn GetCustomObjectEnumerator(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>;
    fn GetCustomReferenceEnumerator(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>;
    fn GetSignatureXml(&mut self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn SetSignatureXml(&mut self, signaturexml: *const u8, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *sigid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureValue(::core::mem::transmute_copy(&signaturehashvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTime() {
                ::core::result::Result::Ok(ok__) => {
                    *sigdatetimestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *timeformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verify(::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *sigstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *policy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSignatureId: GetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignatureValue: GetSignatureValue::<Impl, IMPL_OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Impl, IMPL_OFFSET>,
            GetSigningTime: GetSigningTime::<Impl, IMPL_OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Impl, IMPL_OFFSET>,
            Verify: Verify::<Impl, IMPL_OFFSET>,
            GetPolicy: GetPolicy::<Impl, IMPL_OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Impl, IMPL_OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Impl, IMPL_OFFSET>,
            GetSignatureXml: GetSignatureXml::<Impl, IMPL_OFFSET>,
            SetSignatureXml: SetSignatureXml::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureBlockImpl: Sized {
    fn GetRequests(&mut self) -> ::windows::core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(&mut self) -> ::windows::core::Result<u32>;
    fn GetDocumentName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(&mut self, requestid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlockVtbl {
        unsafe extern "system" fn GetRequests<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequests() {
                ::core::result::Result::Ok(ok__) => {
                    *requests = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *partname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *fixeddocumentindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    *fixeddocumentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRequest(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *signaturerequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRequests: GetRequests::<Impl, IMPL_OFFSET>,
            GetPartName: GetPartName::<Impl, IMPL_OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Impl, IMPL_OFFSET>,
            GetDocumentName: GetDocumentName::<Impl, IMPL_OFFSET>,
            CreateRequest: CreateRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureBlock as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureBlockCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignatureBlock>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureBlockCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlockCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlockCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureBlockCollection as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignature>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManagerImpl: Sized {
    fn LoadPackageFile(&mut self, filename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LoadPackageStream(&mut self, stream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Sign(&mut self, signoptions: ::core::option::Option<IXpsSigningOptions>, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<IXpsSignature>;
    fn GetSignatureOriginPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignatureOriginPartName(&mut self, signatureoriginpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetSignatures(&mut self) -> ::windows::core::Result<IXpsSignatureCollection>;
    fn AddSignatureBlock(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, fixeddocumentindex: u32) -> ::windows::core::Result<IXpsSignatureBlock>;
    fn GetSignatureBlocks(&mut self) -> ::windows::core::Result<IXpsSignatureBlockCollection>;
    fn CreateSigningOptions(&mut self) -> ::windows::core::Result<IXpsSigningOptions>;
    fn SavePackageToFile(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::Result<()>;
    fn SavePackageToStream(&mut self, stream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureManagerVtbl {
        unsafe extern "system" fn LoadPackageFile<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPackageFile(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn LoadPackageStream<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPackageStream(::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn Sign<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sign(::core::mem::transmute(&signoptions), ::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureOriginPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureoriginpartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureOriginPartName(::core::mem::transmute(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatures<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatures() {
                ::core::result::Result::Ok(ok__) => {
                    *signatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSignatureBlock(::core::mem::transmute(&partname), ::core::mem::transmute_copy(&fixeddocumentindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSigningOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *signingoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SavePackageToFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes)).into()
        }
        unsafe extern "system" fn SavePackageToStream<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SavePackageToStream(::core::mem::transmute(&stream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LoadPackageFile: LoadPackageFile::<Impl, IMPL_OFFSET>,
            LoadPackageStream: LoadPackageStream::<Impl, IMPL_OFFSET>,
            Sign: Sign::<Impl, IMPL_OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            GetSignatures: GetSignatures::<Impl, IMPL_OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Impl, IMPL_OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Impl, IMPL_OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Impl, IMPL_OFFSET>,
            SavePackageToFile: SavePackageToFile::<Impl, IMPL_OFFSET>,
            SavePackageToStream: SavePackageToStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureRequestImpl: Sized {
    fn GetIntent(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetIntent(&mut self, intent: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetRequestedSigner(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetRequestedSigner(&mut self, signername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetRequestSignByDate(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetRequestSignByDate(&mut self, datestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSigningLocale(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetSigningLocale(&mut self, place: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSpotLocation(&mut self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows::core::Result<()>;
    fn SetSpotLocation(&mut self, pageindex: i32, x: f32, y: f32) -> ::windows::core::Result<()>;
    fn GetRequestId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSignature(&mut self) -> ::windows::core::Result<IXpsSignature>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequestVtbl {
        unsafe extern "system" fn GetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntent() {
                ::core::result::Result::Ok(ok__) => {
                    *intent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntent(::core::mem::transmute_copy(&intent)).into()
        }
        unsafe extern "system" fn GetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestedSigner() {
                ::core::result::Result::Ok(ok__) => {
                    *signername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedSigner(::core::mem::transmute_copy(&signername)).into()
        }
        unsafe extern "system" fn GetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestSignByDate() {
                ::core::result::Result::Ok(ok__) => {
                    *datestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestSignByDate(::core::mem::transmute_copy(&datestring)).into()
        }
        unsafe extern "system" fn GetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *place = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningLocale(::core::mem::transmute_copy(&place)).into()
        }
        unsafe extern "system" fn GetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&pagepartname), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn GetRequestId<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIntent: GetIntent::<Impl, IMPL_OFFSET>,
            SetIntent: SetIntent::<Impl, IMPL_OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Impl, IMPL_OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Impl, IMPL_OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Impl, IMPL_OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Impl, IMPL_OFFSET>,
            GetSigningLocale: GetSigningLocale::<Impl, IMPL_OFFSET>,
            SetSigningLocale: SetSigningLocale::<Impl, IMPL_OFFSET>,
            GetSpotLocation: GetSpotLocation::<Impl, IMPL_OFFSET>,
            SetSpotLocation: SetSpotLocation::<Impl, IMPL_OFFSET>,
            GetRequestId: GetRequestId::<Impl, IMPL_OFFSET>,
            GetSignature: GetSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureRequest as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureRequestCollectionImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignatureRequest>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureRequestCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequestCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequestCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signaturerequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureRequestCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSigningOptionsImpl: Sized {
    fn GetSignatureId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetSignatureId(&mut self, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSignatureMethod(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetSignatureMethod(&mut self, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDigestMethod(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDigestMethod(&mut self, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSignaturePartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignaturePartName(&mut self, signaturepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetPolicy(&mut self) -> ::windows::core::Result<XPS_SIGN_POLICY>;
    fn SetPolicy(&mut self, policy: XPS_SIGN_POLICY) -> ::windows::core::Result<()>;
    fn GetSigningTimeFormat(&mut self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn SetSigningTimeFormat(&mut self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()>;
    fn GetCustomObjects(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet>;
    fn GetCustomReferences(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateSet>;
    fn GetFlags(&mut self) -> ::windows::core::Result<XPS_SIGN_FLAGS>;
    fn SetFlags(&mut self, flags: XPS_SIGN_FLAGS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSigningOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSigningOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSigningOptionsVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureId(::core::mem::transmute_copy(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturemethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDigestMethod(::core::mem::transmute_copy(&digestmethod)).into()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignaturePartName(::core::mem::transmute(&signaturepartname)).into()
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *policy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *timeformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetCustomObjects<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferences() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSet() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSignatureId: GetSignatureId::<Impl, IMPL_OFFSET>,
            SetSignatureId: SetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Impl, IMPL_OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Impl, IMPL_OFFSET>,
            GetDigestMethod: GetDigestMethod::<Impl, IMPL_OFFSET>,
            SetDigestMethod: SetDigestMethod::<Impl, IMPL_OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Impl, IMPL_OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Impl, IMPL_OFFSET>,
            GetPolicy: GetPolicy::<Impl, IMPL_OFFSET>,
            SetPolicy: SetPolicy::<Impl, IMPL_OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            GetCustomObjects: GetCustomObjects::<Impl, IMPL_OFFSET>,
            GetCustomReferences: GetCustomReferences::<Impl, IMPL_OFFSET>,
            GetCertificateSet: GetCertificateSet::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSigningOptions as ::windows::core::Interface>::IID
    }
}
