#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget_Impl: Sized {
    fn GetXpsOMPackageWriter(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(&mut self) -> ::windows::core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter(::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *xpsfactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
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
pub trait IXpsDocumentPackageTarget3D_Impl: Sized {
    fn GetXpsOMPackageWriter3D(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, modelpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, modeldata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(&mut self) -> ::windows::core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTarget3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTarget3D_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter3D(::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&discardcontrolpartname), ::core::mem::transmute(&modelpartname), ::core::mem::transmute(&modeldata)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMBrush_Impl: Sized + IXpsOMShareable_Impl {
    fn GetOpacity(&mut self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&mut self, opacity: f32) -> ::windows::core::Result<()>;
}
impl IXpsOMBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMBrush_Vtbl {
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *opacity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        Self {
            base: IXpsOMShareable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOpacity: GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMCanvas_Impl: Sized + IXpsOMShareable_Impl + IXpsOMVisual_Impl {
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
impl IXpsOMCanvas_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCanvas_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCanvas_Vtbl {
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *visuals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUseAliasedEdgeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *usealiasededgemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseAliasedEdgeMode(::core::mem::transmute_copy(&usealiasededgemode)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shortdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *longdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)).into()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryLocal(::core::mem::transmute(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryResource(::core::mem::transmute(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMVisual_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMColorProfileResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResource_Vtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMCoreProperties_Impl: Sized + IXpsOMPart_Impl {
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
impl IXpsOMCoreProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCoreProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCoreProperties_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *category = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(::core::mem::transmute_copy(&category)).into()
        }
        unsafe extern "system" fn GetContentStatus<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *contentstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentStatus(::core::mem::transmute_copy(&contentstatus)).into()
        }
        unsafe extern "system" fn GetContentType<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(::core::mem::transmute_copy(&contenttype)).into()
        }
        unsafe extern "system" fn GetCreated<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *created = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreated(::core::mem::transmute_copy(&created)).into()
        }
        unsafe extern "system" fn GetCreator<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *creator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreator(::core::mem::transmute_copy(&creator)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn GetIdentifier<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *identifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdentifier(::core::mem::transmute_copy(&identifier)).into()
        }
        unsafe extern "system" fn GetKeywords<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeywords() {
                ::core::result::Result::Ok(ok__) => {
                    *keywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeywords(::core::mem::transmute_copy(&keywords)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn GetLastModifiedBy<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastModifiedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *lastmodifiedby = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastModifiedBy(::core::mem::transmute_copy(&lastmodifiedby)).into()
        }
        unsafe extern "system" fn GetLastPrinted<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    *lastprinted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastPrinted(::core::mem::transmute_copy(&lastprinted)).into()
        }
        unsafe extern "system" fn GetModified<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModified() {
                ::core::result::Result::Ok(ok__) => {
                    *modified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModified(::core::mem::transmute_copy(&modified)).into()
        }
        unsafe extern "system" fn GetRevision<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *revision = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevision(::core::mem::transmute_copy(&revision)).into()
        }
        unsafe extern "system" fn GetSubject<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubject() {
                ::core::result::Result::Ok(ok__) => {
                    *subject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&subject)).into()
        }
        unsafe extern "system" fn GetTitle<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *title = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&version)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMPart_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMDashCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<XPS_DASH>;
    fn InsertAt(&mut self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
    fn Append(&mut self, dash: *const XPS_DASH) -> ::windows::core::Result<()>;
}
impl IXpsOMDashCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDashCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDashCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *dash = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
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
pub trait IXpsOMDictionary_Impl: Sized {
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
impl IXpsOMDictionary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDictionary_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDictionary_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&entry)).into()
        }
        unsafe extern "system" fn GetByKey<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByKey(::core::mem::transmute_copy(&key), ::core::mem::transmute(&beforeentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *entry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute(&entry)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMDocument_Impl: Sized + IXpsOMPart_Impl {
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
impl IXpsOMDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocument_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageReferences() {
                ::core::result::Result::Ok(ok__) => {
                    *pagereferences = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentStructureResource() {
                ::core::result::Result::Ok(ok__) => {
                    *documentstructureresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentStructureResource(::core::mem::transmute(&documentstructureresource)).into()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlockResources() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMPart_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMDocumentCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMDocument>;
    fn InsertAt(&mut self, index: u32, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
    fn Append(&mut self, document: ::core::option::Option<IXpsOMDocument>) -> ::windows::core::Result<()>;
}
impl IXpsOMDocumentCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&document)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&document)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMDocumentSequence_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPackage>;
    fn GetDocuments(&mut self) -> ::windows::core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(&mut self) -> ::windows::core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&mut self, printticketresource: ::core::option::Option<IXpsOMPrintTicketResource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentSequence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentSequence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentSequence_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *documents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        Self {
            base: IXpsOMPart_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMDocumentStructureResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentStructureResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentStructureResource_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMFontResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetEmbeddingOption(&mut self) -> ::windows::core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResource_Vtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *readerstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute_copy(&embeddingoption), ::core::mem::transmute(&partname)).into()
        }
        unsafe extern "system" fn GetEmbeddingOption<Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT {
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
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMFontResourceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMFontResource>;
    fn SetAt(&mut self, index: u32, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn InsertAt(&mut self, index: u32, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, value: ::core::option::Option<IXpsOMFontResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMGeometry_Impl: Sized + IXpsOMShareable_Impl {
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
impl IXpsOMGeometry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometry_Vtbl {
        unsafe extern "system" fn GetFigures<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFigures() {
                ::core::result::Result::Ok(ok__) => {
                    *figures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillRule() {
                ::core::result::Result::Ok(ok__) => {
                    *fillrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillRule(::core::mem::transmute_copy(&fillrule)).into()
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMShareable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMGeometryFigure_Impl: Sized {
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
impl IXpsOMGeometryFigure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigure_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentData(::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&segmentdata)).into()
        }
        unsafe extern "system" fn GetSegmentTypes<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentTypes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmenttypes)).into()
        }
        unsafe extern "system" fn GetSegmentStrokes<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSegmentStrokes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn SetSegments<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegments(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentdatacount), ::core::mem::transmute_copy(&segmenttypes), ::core::mem::transmute_copy(&segmentdata), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *startpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetIsClosed<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *isclosed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsClosed(::core::mem::transmute_copy(&isclosed)).into()
        }
        unsafe extern "system" fn GetIsFilled<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsFilled() {
                ::core::result::Result::Ok(ok__) => {
                    *isfilled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFilled(::core::mem::transmute_copy(&isfilled)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentDataCount() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentdatacount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSegmentStrokePattern() {
                ::core::result::Result::Ok(ok__) => {
                    *segmentstrokepattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMGeometryFigureCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(&mut self, index: u32, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
    fn Append(&mut self, geometryfigure: ::core::option::Option<IXpsOMGeometryFigure>) -> ::windows::core::Result<()>;
}
impl IXpsOMGeometryFigureCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigureCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *geometryfigure = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&geometryfigure)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&geometryfigure)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMGlyphs_Impl: Sized + IXpsOMShareable_Impl + IXpsOMVisual_Impl {
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
impl IXpsOMGlyphs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphs_Vtbl {
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    *unicodestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *indexcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *glyphmappingcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibitedcaretstopcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProhibitedCaretStops(::core::mem::transmute_copy(&prohibitedcaretstopcount), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *bidilevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    *issideways = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    *devicefontname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyleSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesimulations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyleSimulations(::core::mem::transmute_copy(&stylesimulations)).into()
        }
        unsafe extern "system" fn GetOrigin<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *origin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontRenderingEmSize() {
                ::core::result::Result::Ok(ok__) => {
                    *fontrenderingemsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontRenderingEmSize(::core::mem::transmute_copy(&fontrenderingemsize)).into()
        }
        unsafe extern "system" fn GetFontResource<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontResource() {
                ::core::result::Result::Ok(ok__) => {
                    *fontresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontResource(::core::mem::transmute(&fontresource)).into()
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontFaceIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *fontfaceindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontFaceIndex(::core::mem::transmute_copy(&fontfaceindex)).into()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *fillbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *fillbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLocal(::core::mem::transmute(&fillbrush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetGlyphsEditor<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphsEditor() {
                ::core::result::Result::Ok(ok__) => {
                    *editor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMVisual_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMGlyphsEditor_Impl: Sized {
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
impl IXpsOMGlyphsEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphsEditor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphsEditor_Vtbl {
        unsafe extern "system" fn ApplyEdits<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyEdits().into()
        }
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    *unicodestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicodeString(::core::mem::transmute_copy(&unicodestring)).into()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    *indexcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn SetGlyphIndices<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *glyphmappingcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn SetGlyphMappings<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibitedcaretstopcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *bidilevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBidiLevel(::core::mem::transmute_copy(&bidilevel)).into()
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    *issideways = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSideways(::core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    *devicefontname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IXpsOMGradientBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl {
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
impl IXpsOMGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientBrush_Vtbl {
        unsafe extern "system" fn GetGradientStops<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGradientStops() {
                ::core::result::Result::Ok(ok__) => {
                    *gradientstops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetSpreadMethod<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpreadMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *spreadmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpreadMethod(::core::mem::transmute_copy(&spreadmethod)).into()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *colorinterpolationmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorInterpolationMode(::core::mem::transmute_copy(&colorinterpolationmode)).into()
        }
        Self {
            base: IXpsOMBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMGradientStop_Impl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMGradientBrush>;
    fn GetOffset(&mut self) -> ::windows::core::Result<f32>;
    fn SetOffset(&mut self, offset: f32) -> ::windows::core::Result<()>;
    fn GetColor(&mut self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn SetColor(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMGradientStop>;
}
impl IXpsOMGradientStop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStop_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *offset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetColor<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMGradientStopCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMGradientStop>;
    fn InsertAt(&mut self, index: u32, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
    fn Append(&mut self, stop: ::core::option::Option<IXpsOMGradientStop>) -> ::windows::core::Result<()>;
}
impl IXpsOMGradientStopCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStopCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStopCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *stop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&stop)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&stop)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMImageBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl + IXpsOMTileBrush_Impl {
    fn GetImageResource(&mut self) -> ::windows::core::Result<IXpsOMImageResource>;
    fn SetImageResource(&mut self, imageresource: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn GetColorProfileResource(&mut self) -> ::windows::core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(&mut self, colorprofileresource: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMImageBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMImageBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageBrush_Vtbl {
        unsafe extern "system" fn GetImageResource<Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn GetColorProfileResource<Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResource() {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorProfileResource(::core::mem::transmute(&colorprofileresource)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMImageResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn GetImageType(&mut self) -> ::windows::core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResource_Vtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *readerstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute_copy(&imagetype), ::core::mem::transmute(&partname)).into()
        }
        unsafe extern "system" fn GetImageType<Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT {
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
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMImageResourceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMImageResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMLinearGradientBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl + IXpsOMGradientBrush_Impl {
    fn GetStartPoint(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetStartPoint(&mut self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetEndPoint(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetEndPoint(&mut self, endpoint: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMLinearGradientBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMLinearGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMLinearGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMLinearGradientBrush_Vtbl {
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *startpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetEndPoint<Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *endpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPoint(::core::mem::transmute_copy(&endpoint)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMMatrixTransform_Impl: Sized + IXpsOMShareable_Impl {
    fn GetMatrix(&mut self) -> ::windows::core::Result<XPS_MATRIX>;
    fn SetMatrix(&mut self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMMatrixTransform>;
}
impl IXpsOMMatrixTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMMatrixTransform_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMMatrixTransform_Vtbl {
        unsafe extern "system" fn GetMatrix<Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *matrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMShareable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMNameCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMNameCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMNameCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMNameCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IXpsOMObjectFactory_Impl: Sized {
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
impl IXpsOMObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactory_Vtbl {
        unsafe extern "system" fn CreatePackage<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStoryFragmentsResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *storyfragmentsresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentStructureResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *documentstructureresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSignatureBlockResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResource(::core::mem::transmute(&dictionary), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResourceFromStream(::core::mem::transmute(&dictionarymarkupstream), ::core::mem::transmute(&dictionaryparturi), ::core::mem::transmute(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    *dictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartResources() {
                ::core::result::Result::Ok(ok__) => {
                    *partresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentSequence(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocument(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageReference(::core::mem::transmute_copy(&advisorypagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePage(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute_copy(&language), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream(::core::mem::transmute(&pagemarkupstream), ::core::mem::transmute(&parturi), ::core::mem::transmute(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCanvas() {
                ::core::result::Result::Ok(ok__) => {
                    *canvas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGlyphs(::core::mem::transmute(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *glyphs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeometryFigure(::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *figure = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform(::core::mem::transmute_copy(&matrix)) {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSolidColorBrush(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *solidcolorbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateColorProfileResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImageBrush(::core::mem::transmute(&image), ::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *imagebrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVisualBrush(::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *visualbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImageResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute_copy(&contenttype), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrintTicketResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFontResource(::core::mem::transmute(&acquiredstream), ::core::mem::transmute_copy(&fontembedding), ::core::mem::transmute(&parturi), ::core::mem::transmute_copy(&isobfsourcestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *fontresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGradientStop(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *gradientstop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLinearGradientBrush(::core::mem::transmute(&gradstop1), ::core::mem::transmute(&gradstop2), ::core::mem::transmute_copy(&startpoint), ::core::mem::transmute_copy(&endpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *lineargradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRadialGradientBrush(::core::mem::transmute(&gradstop1), ::core::mem::transmute(&gradstop2), ::core::mem::transmute_copy(&centerpoint), ::core::mem::transmute_copy(&gradientorigin), ::core::mem::transmute_copy(&radiisizes)) {
                ::core::result::Result::Ok(ok__) => {
                    *radialgradientbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCoreProperties(::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *coreproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartUriCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *parturicollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePartUri(::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMObjectFactory1_Impl: Sized + IXpsOMObjectFactory_Impl {
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
impl IXpsOMObjectFactory1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactory1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactory1_Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromFile(::core::mem::transmute_copy(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromStream(::core::mem::transmute(&xpsdocumentstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertHDPhotoToJpegXR(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertJpegXRToHDPhoto(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePackageWriterOnStream1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream1(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::core::mem::transmute(&documentsequencepartname), ::core::mem::transmute(&coreproperties), ::core::mem::transmute(&packagethumbnail), ::core::mem::transmute(&documentsequenceprintticket), ::core::mem::transmute(&discardcontrolpartname), ::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *packagewriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackage1() {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile1(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *package = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePage1(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute_copy(&language), ::core::mem::transmute(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream1(::core::mem::transmute(&pagemarkupstream), ::core::mem::transmute(&parturi), ::core::mem::transmute(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMObjectFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMPackage_Impl: Sized {
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
impl IXpsOMPackage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackage_Vtbl {
        unsafe extern "system" fn GetDocumentSequence<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentSequence() {
                ::core::result::Result::Ok(ok__) => {
                    *documentsequence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentSequence(::core::mem::transmute(&documentsequence)).into()
        }
        unsafe extern "system" fn GetCoreProperties<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCoreProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *coreproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoreProperties(::core::mem::transmute(&coreproperties)).into()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiscardControlPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *discardcontrolparturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscardControlPartName(::core::mem::transmute(&discardcontrolparturi)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn WriteToFile<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn WriteToStream<Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPackage1_Impl: Sized + IXpsOMPackage_Impl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(&mut self, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
    fn WriteToStream1(&mut self, outputstream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackage1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToFile1(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        unsafe extern "system" fn WriteToStream1<Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToStream1(::core::mem::transmute(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMPackage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMPackageTarget_Impl: Sized {
    fn CreateXpsOMPackageWriter(&mut self, documentsequencepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageTarget_Vtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Impl: IXpsOMPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPackageWriter_Impl: Sized {
    fn StartNewDocument(&mut self, documentpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, documentprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, documentstructure: ::core::option::Option<IXpsOMDocumentStructureResource>, signatureblockresources: ::core::option::Option<IXpsOMSignatureBlockResourceCollection>, restrictedfonts: ::core::option::Option<IXpsOMPartUriCollection>) -> ::windows::core::Result<()>;
    fn AddPage(&mut self, page: ::core::option::Option<IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::core::option::Option<IXpsOMPartUriCollection>, storyfragments: ::core::option::Option<IXpsOMStoryFragmentsResource>, pageprintticket: ::core::option::Option<IXpsOMPrintTicketResource>, pagethumbnail: ::core::option::Option<IXpsOMImageResource>) -> ::windows::core::Result<()>;
    fn AddResource(&mut self, resource: ::core::option::Option<IXpsOMResource>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsClosed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriter_Vtbl {
        unsafe extern "system" fn StartNewDocument<Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNewDocument(::core::mem::transmute(&documentpartname), ::core::mem::transmute(&documentprintticket), ::core::mem::transmute(&documentstructure), ::core::mem::transmute(&signatureblockresources), ::core::mem::transmute(&restrictedfonts)).into()
        }
        unsafe extern "system" fn AddPage<Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPage(::core::mem::transmute(&page), ::core::mem::transmute_copy(&advisorypagedimensions), ::core::mem::transmute(&discardableresourceparts), ::core::mem::transmute(&storyfragments), ::core::mem::transmute(&pageprintticket), ::core::mem::transmute(&pagethumbnail)).into()
        }
        unsafe extern "system" fn AddResource<Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResource(::core::mem::transmute(&resource)).into()
        }
        unsafe extern "system" fn Close<Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsClosed<Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPackageWriter3D_Impl: Sized + IXpsOMPackageWriter_Impl {
    fn AddModelTexture(&mut self, texturepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, texturedata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetModelPrintTicket(&mut self, printticketpartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, printticketdata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriter3D_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriter3D_Vtbl {
        unsafe extern "system" fn AddModelTexture<Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddModelTexture(::core::mem::transmute(&texturepartname), ::core::mem::transmute(&texturedata)).into()
        }
        unsafe extern "system" fn SetModelPrintTicket<Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelPrintTicket(::core::mem::transmute(&printticketpartname), ::core::mem::transmute(&printticketdata)).into()
        }
        Self {
            base: IXpsOMPackageWriter_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddModelTexture: AddModelTexture::<Impl, IMPL_OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage_Impl: Sized + IXpsOMPart_Impl {
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
impl IXpsOMPage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPage_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *visuals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pagedimensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetContentBox<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentBox() {
                ::core::result::Result::Ok(ok__) => {
                    *contentbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentBox(::core::mem::transmute_copy(&contentbox)).into()
        }
        unsafe extern "system" fn GetBleedBox<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBleedBox() {
                ::core::result::Result::Ok(ok__) => {
                    *bleedbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBleedBox(::core::mem::transmute_copy(&bleedbox)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ishyperlinktarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlinktarget)).into()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *resourcedictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryLocal(::core::mem::transmute(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    *remotedictionaryresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionaryResource(::core::mem::transmute(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Write<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateUnusedLookupKey(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMPart_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMPage1_Impl: Sized + IXpsOMPart_Impl + IXpsOMPage_Impl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPage1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMPage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentType: GetDocumentType::<Impl, IMPL_OFFSET>,
            Write1: Write1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPage1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMPageReference_Impl: Sized {
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
impl IXpsOMPageReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReference_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *document = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPage() {
                ::core::result::Result::Ok(ok__) => {
                    *page = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPage(::core::mem::transmute(&page)).into()
        }
        unsafe extern "system" fn DiscardPage<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardPage().into()
        }
        unsafe extern "system" fn IsPageLoaded<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPageLoaded() {
                ::core::result::Result::Ok(ok__) => {
                    *ispageloaded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdvisoryPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pagedimensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvisoryPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryFragmentsResource() {
                ::core::result::Result::Ok(ok__) => {
                    *storyfragmentsresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryFragmentsResource(::core::mem::transmute(&storyfragmentsresource)).into()
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    *printticketresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketResource(::core::mem::transmute(&printticketresource)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnailResource(::core::mem::transmute(&imageresource)).into()
        }
        unsafe extern "system" fn CollectLinkTargets<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectLinkTargets() {
                ::core::result::Result::Ok(ok__) => {
                    *linktargets = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectPartResources() {
                ::core::result::Result::Ok(ok__) => {
                    *partresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasRestrictedFonts() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictedfonts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPageReferenceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMPageReference>;
    fn InsertAt(&mut self, index: u32, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
    fn Append(&mut self, pagereference: ::core::option::Option<IXpsOMPageReference>) -> ::windows::core::Result<()>;
}
impl IXpsOMPageReferenceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReferenceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReferenceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pagereference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&pagereference)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&pagereference)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPart_Impl: Sized {
    fn GetPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPart_Vtbl {
        unsafe extern "system" fn GetPartName<Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPartResources_Impl: Sized {
    fn GetFontResources(&mut self) -> ::windows::core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(&mut self) -> ::windows::core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(&mut self) -> ::windows::core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(&mut self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl IXpsOMPartResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartResources_Vtbl {
        unsafe extern "system" fn GetFontResources<Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontResources() {
                ::core::result::Result::Ok(ok__) => {
                    *fontresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageResources() {
                ::core::result::Result::Ok(ok__) => {
                    *imageresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResources() {
                ::core::result::Result::Ok(ok__) => {
                    *colorprofileresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPartUriCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(&mut self, index: u32, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
    fn Append(&mut self, parturi: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartUriCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartUriCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartUriCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *parturi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&parturi)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&parturi)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMPath_Impl: Sized + IXpsOMShareable_Impl + IXpsOMVisual_Impl {
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
impl IXpsOMPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPath_Vtbl {
        unsafe extern "system" fn GetGeometry<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *geometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometryLocal(::core::mem::transmute(&geometry)).into()
        }
        unsafe extern "system" fn GetGeometryLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeometryLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *shortdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *longdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)).into()
        }
        unsafe extern "system" fn GetSnapsToPixels<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapsToPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *snapstopixels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapsToPixels(::core::mem::transmute_copy(&snapstopixels)).into()
        }
        unsafe extern "system" fn GetStrokeBrush<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrushLocal(::core::mem::transmute(&brush)).into()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeBrushLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn GetStrokeDashes<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashes() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashcap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashCap(::core::mem::transmute_copy(&strokedashcap)).into()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *strokedashoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeDashOffset(::core::mem::transmute_copy(&strokedashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeStartLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokestartlinecap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeStartLineCap(::core::mem::transmute_copy(&strokestartlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeEndLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    *strokeendlinecap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeEndLineCap(::core::mem::transmute_copy(&strokeendlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    *strokelinejoin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeLineJoin(::core::mem::transmute_copy(&strokelinejoin)).into()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *strokemiterlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeMiterLimit(::core::mem::transmute_copy(&strokemiterlimit)).into()
        }
        unsafe extern "system" fn GetStrokeThickness<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *strokethickness = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrokeThickness(::core::mem::transmute_copy(&strokethickness)).into()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *brush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLocal(::core::mem::transmute(&brush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFillBrushLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMVisual_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMPrintTicketResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPrintTicketResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPrintTicketResource_Vtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMRadialGradientBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl + IXpsOMGradientBrush_Impl {
    fn GetCenter(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetCenter(&mut self, center: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn GetRadiiSizes(&mut self) -> ::windows::core::Result<XPS_SIZE>;
    fn SetRadiiSizes(&mut self, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<()>;
    fn GetGradientOrigin(&mut self) -> ::windows::core::Result<XPS_POINT>;
    fn SetGradientOrigin(&mut self, origin: *const XPS_POINT) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMRadialGradientBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMRadialGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRadialGradientBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRadialGradientBrush_Vtbl {
        unsafe extern "system" fn GetCenter<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *center = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(::core::mem::transmute_copy(&center)).into()
        }
        unsafe extern "system" fn GetRadiiSizes<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadiiSizes() {
                ::core::result::Result::Ok(ok__) => {
                    *radiisizes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadiiSizes(::core::mem::transmute_copy(&radiisizes)).into()
        }
        unsafe extern "system" fn GetGradientOrigin<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGradientOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *origin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGradientOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMRemoteDictionaryResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetDictionary(&mut self) -> ::windows::core::Result<IXpsOMDictionary>;
    fn SetDictionary(&mut self, dictionary: ::core::option::Option<IXpsOMDictionary>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResource_Vtbl {
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *dictionary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictionary(::core::mem::transmute(&dictionary)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDictionary: GetDictionary::<Impl, IMPL_OFFSET>,
            SetDictionary: SetDictionary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl + IXpsOMRemoteDictionaryResource_Impl {
    fn GetDocumentType(&mut self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&mut self, stream: ::core::option::Option<super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResource1_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write1(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base: IXpsOMRemoteDictionaryResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDocumentType: GetDocumentType::<Impl, IMPL_OFFSET>,
            Write1: Write1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMRemoteDictionaryResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMResource_Impl: Sized + IXpsOMPart_Impl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMResource_Vtbl {
        Self { base: IXpsOMPart_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMResource as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMShareable_Impl: Sized {
    fn GetOwner(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetType(&mut self) -> ::windows::core::Result<XPS_OBJECT_TYPE>;
}
impl IXpsOMShareable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMShareable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMShareable_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT {
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
pub trait IXpsOMSignatureBlockResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMDocument>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResource_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMSignatureBlockResourceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(&mut self, index: u32, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn Append(&mut self, signatureblockresource: ::core::option::Option<IXpsOMSignatureBlockResource>) -> ::windows::core::Result<()>;
    fn GetByPartName(&mut self, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblockresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&signatureblockresource)).into()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMSolidColorBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl {
    fn GetColor(&mut self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn SetColor(&mut self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMSolidColorBrush>;
}
impl IXpsOMSolidColorBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSolidColorBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSolidColorBrush_Vtbl {
        unsafe extern "system" fn GetColor<Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMStoryFragmentsResource_Impl: Sized + IXpsOMPart_Impl + IXpsOMResource_Impl {
    fn GetOwner(&mut self) -> ::windows::core::Result<IXpsOMPageReference>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetContent(&mut self, sourcestream: ::core::option::Option<super::super::System::Com::IStream>, partname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMStoryFragmentsResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMStoryFragmentsResource_Vtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *owner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(::core::mem::transmute(&sourcestream), ::core::mem::transmute(&partname)).into()
        }
        Self {
            base: IXpsOMResource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMThumbnailGenerator_Impl: Sized {
    fn GenerateThumbnail(&mut self, page: ::core::option::Option<IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::core::option::Option<super::Packaging::Opc::IOpcPartUri>) -> ::windows::core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMThumbnailGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMThumbnailGenerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMThumbnailGenerator_Vtbl {
        unsafe extern "system" fn GenerateThumbnail<Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsOMTileBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl {
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
impl IXpsOMTileBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMTileBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMTileBrush_Vtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *transform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetViewbox<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewbox() {
                ::core::result::Result::Ok(ok__) => {
                    *viewbox = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewbox(::core::mem::transmute_copy(&viewbox)).into()
        }
        unsafe extern "system" fn GetViewport<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *viewport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewport(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn GetTileMode<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTileMode() {
                ::core::result::Result::Ok(ok__) => {
                    *tilemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileMode(::core::mem::transmute_copy(&tilemode)).into()
        }
        Self {
            base: IXpsOMBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMVisual_Impl: Sized + IXpsOMShareable_Impl {
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
impl IXpsOMVisual_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisual_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisual_Vtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *matrixtransform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLocal(::core::mem::transmute(&matrixtransform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetClipGeometry<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    *clipgeometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *clipgeometry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipGeometryLocal(::core::mem::transmute(&clipgeometry)).into()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClipGeometryLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *opacity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *opacitymaskbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *opacitymaskbrush = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMaskBrushLocal(::core::mem::transmute(&opacitymaskbrush)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityMaskBrushLookup(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ishyperlink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlink)).into()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHyperlinkNavigateUri() {
                ::core::result::Result::Ok(ok__) => {
                    *hyperlinkuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHyperlinkNavigateUri(::core::mem::transmute(&hyperlinkuri)).into()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *language = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&language)).into()
        }
        Self {
            base: IXpsOMShareable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMVisualBrush_Impl: Sized + IXpsOMShareable_Impl + IXpsOMBrush_Impl + IXpsOMTileBrush_Impl {
    fn GetVisual(&mut self) -> ::windows::core::Result<IXpsOMVisual>;
    fn GetVisualLocal(&mut self) -> ::windows::core::Result<IXpsOMVisual>;
    fn SetVisualLocal(&mut self, visual: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn GetVisualLookup(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetVisualLookup(&mut self, lookup: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IXpsOMVisualBrush>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMVisualBrush_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualBrush_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualBrush_Vtbl {
        unsafe extern "system" fn GetVisual<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisual() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *visual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualLocal(::core::mem::transmute(&visual)).into()
        }
        unsafe extern "system" fn GetVisualLookup<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualLookup() {
                ::core::result::Result::Ok(ok__) => {
                    *lookup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisualLookup(::core::mem::transmute_copy(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IXpsOMVisualCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsOMVisual>;
    fn InsertAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn SetAt(&mut self, index: u32, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
    fn Append(&mut self, object: ::core::option::Option<IXpsOMVisual>) -> ::windows::core::Result<()>;
}
impl IXpsOMVisualCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsSignature_Impl: Sized {
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
impl IXpsSignature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignature_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignature_Vtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *sigid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureValue(::core::mem::transmute_copy(&signaturehashvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTime() {
                ::core::result::Result::Ok(ok__) => {
                    *sigdatetimestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *timeformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verify(::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *sigstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *policy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetSignatureXml<Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureBlock_Impl: Sized {
    fn GetRequests(&mut self) -> ::windows::core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(&mut self) -> ::windows::core::Result<u32>;
    fn GetDocumentName(&mut self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(&mut self, requestid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureBlock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlock_Vtbl {
        unsafe extern "system" fn GetRequests<Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequests() {
                ::core::result::Result::Ok(ok__) => {
                    *requests = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *partname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *fixeddocumentindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    *fixeddocumentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureBlockCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignatureBlock>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureBlockCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlockCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlockCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignature>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureManager_Impl: Sized {
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
impl IXpsSignatureManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureManager_Vtbl {
        unsafe extern "system" fn LoadPackageFile<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPackageFile(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn LoadPackageStream<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPackageStream(::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn Sign<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sign(::core::mem::transmute(&signoptions), ::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *signature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureOriginPartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureoriginpartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureOriginPartName(::core::mem::transmute(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatures<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatures() {
                ::core::result::Result::Ok(ok__) => {
                    *signatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSignatureBlock(::core::mem::transmute(&partname), ::core::mem::transmute_copy(&fixeddocumentindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSigningOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *signingoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SavePackageToFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes)).into()
        }
        unsafe extern "system" fn SavePackageToStream<Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureRequest_Impl: Sized {
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
impl IXpsSignatureRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequest_Vtbl {
        unsafe extern "system" fn GetIntent<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntent() {
                ::core::result::Result::Ok(ok__) => {
                    *intent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntent(::core::mem::transmute_copy(&intent)).into()
        }
        unsafe extern "system" fn GetRequestedSigner<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestedSigner() {
                ::core::result::Result::Ok(ok__) => {
                    *signername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedSigner(::core::mem::transmute_copy(&signername)).into()
        }
        unsafe extern "system" fn GetRequestSignByDate<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestSignByDate() {
                ::core::result::Result::Ok(ok__) => {
                    *datestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestSignByDate(::core::mem::transmute_copy(&datestring)).into()
        }
        unsafe extern "system" fn GetSigningLocale<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *place = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningLocale(::core::mem::transmute_copy(&place)).into()
        }
        unsafe extern "system" fn GetSpotLocation<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&pagepartname), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetSpotLocation<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn GetRequestId<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXpsSignatureRequestCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, index: u32) -> ::windows::core::Result<IXpsSignatureRequest>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
}
impl IXpsSignatureRequestCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequestCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequestCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *signaturerequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
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
pub trait IXpsSigningOptions_Impl: Sized {
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
impl IXpsSigningOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSigningOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *signatureid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureId(::core::mem::transmute_copy(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturemethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *digestmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDigestMethod(::core::mem::transmute_copy(&digestmethod)).into()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    *signaturepartname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignaturePartName(::core::mem::transmute(&signaturepartname)).into()
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *policy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *timeformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetCustomObjects<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *customobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomReferences() {
                ::core::result::Result::Ok(ok__) => {
                    *customreferenceset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateSet() {
                ::core::result::Result::Ok(ok__) => {
                    *certificateset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
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
