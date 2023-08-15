#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget_Impl: Sized {
    fn GetXpsOMPackageWriter(&self, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(&self) -> ::windows_core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsDocumentPackageTarget {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>() -> IXpsDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsOMPackageWriter(::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsOMFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpsfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
            GetXpsType: GetXpsType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3D_Impl: Sized {
    fn GetXpsOMPackageWriter3D(&self, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, modelpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, modeldata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(&self) -> ::windows_core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsDocumentPackageTarget3D {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3D_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>() -> IXpsDocumentPackageTarget3D_Vtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, modelpartname: *mut ::core::ffi::c_void, modeldata: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsOMPackageWriter3D(::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&discardcontrolpartname), ::windows_core::from_raw_borrowed(&modelpartname), ::windows_core::from_raw_borrowed(&modeldata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXpsOMFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpsfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget3D as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMBrush_Impl: Sized + IXpsOMShareable_Impl {
    fn GetOpacity(&self) -> ::windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMBrush {}
impl IXpsOMBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>() -> IXpsOMBrush_Vtbl {
        unsafe extern "system" fn GetOpacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMCanvas_Impl: Sized + IXpsOMVisual_Impl {
    fn GetVisuals(&self) -> ::windows_core::Result<IXpsOMVisualCollection>;
    fn GetUseAliasedEdgeMode(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUseAliasedEdgeMode(&self, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMCanvas>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMCanvas {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMCanvas_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>() -> IXpsOMCanvas_Vtbl {
        unsafe extern "system" fn GetVisuals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visuals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUseAliasedEdgeMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usealiasededgemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseAliasedEdgeMode(::core::mem::transmute_copy(&usealiasededgemode)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessibilityShortDescription(::core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(longdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessibilityLongDescription(::core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictionaryLocal(::windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictionaryResource(::windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canvas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMCanvas as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMVisual as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMColorProfileResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>() -> IXpsOMColorProfileResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(&self, index: u32, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Append(&self, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMColorProfileResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMColorProfileResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByPartName(::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResourceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMCoreProperties_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPackage>;
    fn GetCategory(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCategory(&self, category: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetContentStatus(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetContentStatus(&self, contentstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetContentType(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetContentType(&self, contenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCreated(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetCreator(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCreator(&self, creator: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDescription(&self, description: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIdentifier(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIdentifier(&self, identifier: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetKeywords(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetKeywords(&self, keywords: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(&self, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLastModifiedBy(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLastModifiedBy(&self, lastmodifiedby: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLastPrinted(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetModified(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetRevision(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRevision(&self, revision: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSubject(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSubject(&self, subject: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetTitle(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTitle(&self, title: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetVersion(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetVersion(&self, version: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMCoreProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMCoreProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMCoreProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>() -> IXpsOMCoreProperties_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(category, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCategory(::core::mem::transmute(&category)).into()
        }
        unsafe extern "system" fn GetContentStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentStatus(::core::mem::transmute(&contentstatus)).into()
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentType(::core::mem::transmute(&contenttype)).into()
        }
        unsafe extern "system" fn GetCreated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCreated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(created, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreated(::core::mem::transmute_copy(&created)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(creator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreator(::core::mem::transmute(&creator)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(identifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIdentifier(::core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn GetKeywords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeywords() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keywords, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeywords(::core::mem::transmute(&keywords)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguage(::core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetLastModifiedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastModifiedBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodifiedby, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastModifiedBy(::core::mem::transmute(&lastmodifiedby)).into()
        }
        unsafe extern "system" fn GetLastPrinted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastprinted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastPrinted(::core::mem::transmute_copy(&lastprinted)).into()
        }
        unsafe extern "system" fn GetModified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModified(::core::mem::transmute_copy(&modified)).into()
        }
        unsafe extern "system" fn GetRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRevision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(revision, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRevision(::core::mem::transmute(&revision)).into()
        }
        unsafe extern "system" fn GetSubject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubject(::core::mem::transmute(&subject)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTitle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
            GetContentStatus: GetContentStatus::<Identity, Impl, OFFSET>,
            SetContentStatus: SetContentStatus::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            SetContentType: SetContentType::<Identity, Impl, OFFSET>,
            GetCreated: GetCreated::<Identity, Impl, OFFSET>,
            SetCreated: SetCreated::<Identity, Impl, OFFSET>,
            GetCreator: GetCreator::<Identity, Impl, OFFSET>,
            SetCreator: SetCreator::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            SetIdentifier: SetIdentifier::<Identity, Impl, OFFSET>,
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            SetKeywords: SetKeywords::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Identity, Impl, OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Identity, Impl, OFFSET>,
            GetLastPrinted: GetLastPrinted::<Identity, Impl, OFFSET>,
            SetLastPrinted: SetLastPrinted::<Identity, Impl, OFFSET>,
            GetModified: GetModified::<Identity, Impl, OFFSET>,
            SetModified: SetModified::<Identity, Impl, OFFSET>,
            GetRevision: GetRevision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            GetSubject: GetSubject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMCoreProperties as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMDashCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<XPS_DASH>;
    fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
    fn Append(&self, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMDashCollection {}
impl IXpsOMDashCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>() -> IXpsOMDashCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dash, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute_copy(&dash)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDashCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMDictionary_Impl: Sized {
    fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn GetByKey(&self, key: &::windows_core::PCWSTR, beforeentry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<IXpsOMShareable>;
    fn GetIndex(&self, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<u32>;
    fn Append(&self, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn InsertAt(&self, index: u32, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMDictionary>;
}
impl ::windows_core::RuntimeName for IXpsOMDictionary {}
impl IXpsOMDictionary_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>() -> IXpsOMDictionary_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&entry)).into()
        }
        unsafe extern "system" fn GetByKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, beforeentry: *mut ::core::ffi::c_void, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByKey(::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&beforeentry)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndex(::windows_core::from_raw_borrowed(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetByKey: GetByKey::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDictionary as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocument_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn GetPageReferences(&self) -> ::windows_core::Result<IXpsOMPageReferenceCollection>;
    fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
    fn GetDocumentStructureResource(&self) -> ::windows_core::Result<IXpsOMDocumentStructureResource>;
    fn SetDocumentStructureResource(&self, documentstructureresource: ::core::option::Option<&IXpsOMDocumentStructureResource>) -> ::windows_core::Result<()>;
    fn GetSignatureBlockResources(&self) -> ::windows_core::Result<IXpsOMSignatureBlockResourceCollection>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMDocument>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMDocument {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>() -> IXpsOMDocument_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereferences: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPageReferences() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereferences, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicketResource(::windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentStructureResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentstructureresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentStructureResource(::windows_core::from_raw_borrowed(&documentstructureresource)).into()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureBlockResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPageReferences: GetPageReferences::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Identity, Impl, OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Identity, Impl, OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDocument as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMDocumentCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMDocument>;
    fn InsertAt(&self, index: u32, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
    fn Append(&self, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMDocumentCollection {}
impl IXpsOMDocumentCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>() -> IXpsOMDocumentCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&document)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&document)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentSequence_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPackage>;
    fn GetDocuments(&self) -> ::windows_core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMDocumentSequence {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentSequence_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>() -> IXpsOMDocumentSequence_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicketResource(::windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentSequence as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMDocumentStructureResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>() -> IXpsOMDocumentStructureResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentStructureResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetEmbeddingOption(&self) -> ::windows_core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMFontResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>() -> IXpsOMFontResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readerstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::core::mem::transmute_copy(&embeddingoption), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetEmbeddingOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbeddingOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(embeddingoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMFontResource>;
    fn SetAt(&self, index: u32, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn Append(&self, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn GetByPartName(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMFontResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMFontResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByPartName(::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResourceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMGeometry_Impl: Sized + IXpsOMShareable_Impl {
    fn GetFigures(&self) -> ::windows_core::Result<IXpsOMGeometryFigureCollection>;
    fn GetFillRule(&self) -> ::windows_core::Result<XPS_FILL_RULE>;
    fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows_core::Result<()>;
    fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(&self, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMGeometry>;
}
impl ::windows_core::RuntimeName for IXpsOMGeometry {}
impl IXpsOMGeometry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>() -> IXpsOMGeometry_Vtbl {
        unsafe extern "system" fn GetFigures<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFigures() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(figures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFillRule(::core::mem::transmute_copy(&fillrule)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLocal(::windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLookup(::core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFigures: GetFigures::<Identity, Impl, OFFSET>,
            GetFillRule: GetFillRule::<Identity, Impl, OFFSET>,
            SetFillRule: SetFillRule::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometry as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryFigure_Impl: Sized {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::Result<()>;
    fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::Result<()>;
    fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetStartPoint(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetIsClosed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsClosed(&self, isclosed: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetIsFilled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsFilled(&self, isfilled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSegmentCount(&self) -> ::windows_core::Result<u32>;
    fn GetSegmentDataCount(&self) -> ::windows_core::Result<u32>;
    fn GetSegmentStrokePattern(&self) -> ::windows_core::Result<XPS_SEGMENT_STROKE_PATTERN>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMGeometryFigure>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IXpsOMGeometryFigure {}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGeometryFigure_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>() -> IXpsOMGeometryFigure_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSegmentData(::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&segmentdata)).into()
        }
        unsafe extern "system" fn GetSegmentTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSegmentTypes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmenttypes)).into()
        }
        unsafe extern "system" fn GetSegmentStrokes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSegmentStrokes(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn SetSegments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSegments(::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentdatacount), ::core::mem::transmute_copy(&segmenttypes), ::core::mem::transmute_copy(&segmentdata), ::core::mem::transmute_copy(&segmentstrokes)).into()
        }
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetIsClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isclosed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsClosed(::core::mem::transmute_copy(&isclosed)).into()
        }
        unsafe extern "system" fn GetIsFilled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsFilled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isfilled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsFilled(::core::mem::transmute_copy(&isfilled)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSegmentCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSegmentDataCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentdatacount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSegmentStrokePattern() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentstrokepattern, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryfigure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, Impl, OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Identity, Impl, OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Identity, Impl, OFFSET>,
            SetSegments: SetSegments::<Identity, Impl, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetIsClosed: GetIsClosed::<Identity, Impl, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, Impl, OFFSET>,
            GetIsFilled: GetIsFilled::<Identity, Impl, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, Impl, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, Impl, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, Impl, OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigure as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMGeometryFigureCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(&self, index: u32, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
    fn Append(&self, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMGeometryFigureCollection {}
impl IXpsOMGeometryFigureCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>() -> IXpsOMGeometryFigureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryfigure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&geometryfigure)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigureCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMGlyphs_Impl: Sized + IXpsOMVisual_Impl {
    fn GetUnicodeString(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetGlyphIndexCount(&self) -> ::windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> ::windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> ::windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()>;
    fn GetBidiLevel(&self) -> ::windows_core::Result<u32>;
    fn GetIsSideways(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDeviceFontName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetStyleSimulations(&self) -> ::windows_core::Result<XPS_STYLE_SIMULATION>;
    fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::Result<()>;
    fn GetOrigin(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetFontRenderingEmSize(&self) -> ::windows_core::Result<f32>;
    fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows_core::Result<()>;
    fn GetFontResource(&self) -> ::windows_core::Result<IXpsOMFontResource>;
    fn SetFontResource(&self, fontresource: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn GetFontFaceIndex(&self) -> ::windows_core::Result<i16>;
    fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows_core::Result<()>;
    fn GetFillBrush(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, fillbrush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetGlyphsEditor(&self) -> ::windows_core::Result<IXpsOMGlyphsEditor>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMGlyphs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMGlyphs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMGlyphs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>() -> IXpsOMGlyphs_Vtbl {
        unsafe extern "system" fn GetUnicodeString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicodestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(indexcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphmappingcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibitedcaretstopcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProhibitedCaretStops(::core::mem::transmute_copy(&prohibitedcaretstopcount), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bidilevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issideways, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicefontname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStyleSimulations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesimulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStyleSimulations(::core::mem::transmute_copy(&stylesimulations)).into()
        }
        unsafe extern "system" fn GetOrigin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(origin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontRenderingEmSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontrenderingemsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontRenderingEmSize(::core::mem::transmute_copy(&fontrenderingemsize)).into()
        }
        unsafe extern "system" fn GetFontResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontResource(::windows_core::from_raw_borrowed(&fontresource)).into()
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontFaceIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfaceindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontFaceIndex(::core::mem::transmute_copy(&fontfaceindex)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFillBrushLocal(::windows_core::from_raw_borrowed(&fillbrush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFillBrushLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetGlyphsEditor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphsEditor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(editor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Identity, Impl, OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Identity, Impl, OFFSET>,
            GetOrigin: GetOrigin::<Identity, Impl, OFFSET>,
            SetOrigin: SetOrigin::<Identity, Impl, OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            GetFontResource: GetFontResource::<Identity, Impl, OFFSET>,
            SetFontResource: SetFontResource::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphs as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMVisual as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGlyphsEditor_Impl: Sized {
    fn ApplyEdits(&self) -> ::windows_core::Result<()>;
    fn GetUnicodeString(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetUnicodeString(&self, unicodestring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetGlyphIndexCount(&self) -> ::windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> ::windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> ::windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()>;
    fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::Result<()>;
    fn GetBidiLevel(&self) -> ::windows_core::Result<u32>;
    fn SetBidiLevel(&self, bidilevel: u32) -> ::windows_core::Result<()>;
    fn GetIsSideways(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsSideways(&self, issideways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDeviceFontName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDeviceFontName(&self, devicefontname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IXpsOMGlyphsEditor {}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGlyphsEditor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>() -> IXpsOMGlyphsEditor_Vtbl {
        unsafe extern "system" fn ApplyEdits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyEdits().into()
        }
        unsafe extern "system" fn GetUnicodeString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnicodeString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicodestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnicodeString(::core::mem::transmute(&unicodestring)).into()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphIndexCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(indexcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn SetGlyphIndices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGlyphIndices(::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlyphMappingCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphmappingcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn SetGlyphMappings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGlyphMappings(::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProhibitedCaretStopCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibitedcaretstopcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProhibitedCaretStops(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into()
        }
        unsafe extern "system" fn GetBidiLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBidiLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bidilevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBidiLevel(::core::mem::transmute_copy(&bidilevel)).into()
        }
        unsafe extern "system" fn GetIsSideways<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issideways, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsSideways(::core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceFontName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicefontname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceFontName(::core::mem::transmute(&devicefontname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplyEdits: ApplyEdits::<Identity, Impl, OFFSET>,
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            SetUnicodeString: SetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            SetIsSideways: SetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphsEditor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMGradientBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetGradientStops(&self) -> ::windows_core::Result<IXpsOMGradientStopCollection>;
    fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSpreadMethod(&self) -> ::windows_core::Result<XPS_SPREAD_METHOD>;
    fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::Result<()>;
    fn GetColorInterpolationMode(&self) -> ::windows_core::Result<XPS_COLOR_INTERPOLATION>;
    fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMGradientBrush {}
impl IXpsOMGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMGradientBrush_Vtbl {
        unsafe extern "system" fn GetGradientStops<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGradientStops() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLocal(::windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetSpreadMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpreadMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(spreadmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpreadMethod(::core::mem::transmute_copy(&spreadmethod)).into()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColorInterpolationMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorinterpolationmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorInterpolationMode(::core::mem::transmute_copy(&colorinterpolationmode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGradientStops: GetGradientStops::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Identity, Impl, OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Identity, Impl, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, Impl, OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMGradientStop_Impl: Sized {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMGradientBrush>;
    fn GetOffset(&self) -> ::windows_core::Result<f32>;
    fn SetOffset(&self, offset: f32) -> ::windows_core::Result<()>;
    fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMGradientStop>;
}
impl ::windows_core::RuntimeName for IXpsOMGradientStop {}
impl IXpsOMGradientStop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>() -> IXpsOMGradientStop_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOffset(::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColor(::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStop as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMGradientStopCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMGradientStop>;
    fn InsertAt(&self, index: u32, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
    fn Append(&self, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMGradientStopCollection {}
impl IXpsOMGradientStopCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>() -> IXpsOMGradientStopCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&stop)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&stop)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStopCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMImageBrush_Impl: Sized + IXpsOMTileBrush_Impl {
    fn GetImageResource(&self) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetImageResource(&self, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn GetColorProfileResource(&self) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(&self, colorprofileresource: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMImageBrush>;
}
impl ::windows_core::RuntimeName for IXpsOMImageBrush {}
impl IXpsOMImageBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>() -> IXpsOMImageBrush_Vtbl {
        unsafe extern "system" fn GetImageResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImageResource(::windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn GetColorProfileResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColorProfileResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorProfileResource(::windows_core::from_raw_borrowed(&colorprofileresource)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagebrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImageResource: GetImageResource::<Identity, Impl, OFFSET>,
            SetImageResource: SetImageResource::<Identity, Impl, OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Identity, Impl, OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMImageBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMTileBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetImageType(&self) -> ::windows_core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMImageResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>() -> IXpsOMImageResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readerstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::core::mem::transmute_copy(&imagetype), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        unsafe extern "system" fn GetImageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetImageType: GetImageType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMImageResource>;
    fn InsertAt(&self, index: u32, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn Append(&self, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMImageResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMImageResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByPartName(::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResourceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMLinearGradientBrush_Impl: Sized + IXpsOMGradientBrush_Impl {
    fn GetStartPoint(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetEndPoint(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMLinearGradientBrush>;
}
impl ::windows_core::RuntimeName for IXpsOMLinearGradientBrush {}
impl IXpsOMLinearGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMLinearGradientBrush_Vtbl {
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStartPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartPoint(::core::mem::transmute_copy(&startpoint)).into()
        }
        unsafe extern "system" fn GetEndPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(endpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndPoint(::core::mem::transmute_copy(&endpoint)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineargradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, Impl, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMLinearGradientBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMGradientBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMMatrixTransform_Impl: Sized + IXpsOMShareable_Impl {
    fn GetMatrix(&self) -> ::windows_core::Result<XPS_MATRIX>;
    fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
}
impl ::windows_core::RuntimeName for IXpsOMMatrixTransform {}
impl IXpsOMMatrixTransform_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>() -> IXpsOMMatrixTransform_Vtbl {
        unsafe extern "system" fn GetMatrix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrix(::core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMatrix: GetMatrix::<Identity, Impl, OFFSET>,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMMatrixTransform as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMNameCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for IXpsOMNameCollection {}
impl IXpsOMNameCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>() -> IXpsOMNameCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMNameCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory_Impl: Sized {
    fn CreatePackage(&self) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromFile(&self, filename: &::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromStream(&self, stream: ::core::option::Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreateStoryFragmentsResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn CreateDocumentStructureResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocumentStructureResource>;
    fn CreateSignatureBlockResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
    fn CreateRemoteDictionaryResource(&self, dictionary: ::core::option::Option<&IXpsOMDictionary>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreateRemoteDictionaryResourceFromStream(&self, dictionarymarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, dictionaryparturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreatePartResources(&self) -> ::windows_core::Result<IXpsOMPartResources>;
    fn CreateDocumentSequence(&self, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn CreateDocument(&self, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocument>;
    fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMPageReference>;
    fn CreatePage(&self, pagedimensions: *const XPS_SIZE, language: &::windows_core::PCWSTR, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPage>;
    fn CreatePageFromStream(&self, pagemarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPage>;
    fn CreateCanvas(&self) -> ::windows_core::Result<IXpsOMCanvas>;
    fn CreateGlyphs(&self, fontresource: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<IXpsOMGlyphs>;
    fn CreatePath(&self) -> ::windows_core::Result<IXpsOMPath>;
    fn CreateGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMGeometryFigure>;
    fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn CreateSolidColorBrush(&self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<IXpsOMSolidColorBrush>;
    fn CreateColorProfileResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn CreateImageBrush(&self, image: ::core::option::Option<&IXpsOMImageResource>, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMImageBrush>;
    fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMVisualBrush>;
    fn CreateImageResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, contenttype: XPS_IMAGE_TYPE, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
    fn CreatePrintTicketResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn CreateFontResource(&self, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, fontembedding: XPS_FONT_EMBEDDING, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, isobfsourcestream: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMFontResource>;
    fn CreateGradientStop(&self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>, offset: f32) -> ::windows_core::Result<IXpsOMGradientStop>;
    fn CreateLinearGradientBrush(&self, gradstop1: ::core::option::Option<&IXpsOMGradientStop>, gradstop2: ::core::option::Option<&IXpsOMGradientStop>, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMLinearGradientBrush>;
    fn CreateRadialGradientBrush(&self, gradstop1: ::core::option::Option<&IXpsOMGradientStop>, gradstop2: ::core::option::Option<&IXpsOMGradientStop>, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMRadialGradientBrush>;
    fn CreateCoreProperties(&self, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMCoreProperties>;
    fn CreateDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn CreatePartUriCollection(&self) -> ::windows_core::Result<IXpsOMPartUriCollection>;
    fn CreatePackageWriterOnFile(&self, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream(&self, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePartUri(&self, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateReadOnlyStreamOnFile(&self, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMObjectFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>() -> IXpsOMObjectFactory_Vtbl {
        unsafe extern "system" fn CreatePackage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageFromFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageFromStream(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStoryFragmentsResource(::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyfragmentsresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentStructureResource(::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentstructureresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSignatureBlockResource(::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRemoteDictionaryResource(::windows_core::from_raw_borrowed(&dictionary), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, dictionaryparturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRemoteDictionaryResourceFromStream(::windows_core::from_raw_borrowed(&dictionarymarkupstream), ::windows_core::from_raw_borrowed(&dictionaryparturi), ::windows_core::from_raw_borrowed(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePartResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentSequence(::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocument(::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePageReference(::core::mem::transmute_copy(&advisorypagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePage(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute(&language), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePageFromStream(::windows_core::from_raw_borrowed(&pagemarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCanvas() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canvas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGlyphs(::windows_core::from_raw_borrowed(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGeometryFigure(::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(figure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMatrixTransform(::core::mem::transmute_copy(&matrix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSolidColorBrush(::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(solidcolorbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateColorProfileResource(::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateImageBrush(::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagebrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVisualBrush(::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visualbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateImageResource(::windows_core::from_raw_borrowed(&acquiredstream), ::core::mem::transmute_copy(&contenttype), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePrintTicketResource(::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut ::core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFontResource(::windows_core::from_raw_borrowed(&acquiredstream), ::core::mem::transmute_copy(&fontembedding), ::windows_core::from_raw_borrowed(&parturi), ::core::mem::transmute_copy(&isobfsourcestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, offset: f32, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGradientStop(::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLinearGradientBrush(::windows_core::from_raw_borrowed(&gradstop1), ::windows_core::from_raw_borrowed(&gradstop2), ::core::mem::transmute_copy(&startpoint), ::core::mem::transmute_copy(&endpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineargradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRadialGradientBrush(::windows_core::from_raw_borrowed(&gradstop1), ::windows_core::from_raw_borrowed(&gradstop2), ::core::mem::transmute_copy(&centerpoint), ::core::mem::transmute_copy(&gradientorigin), ::core::mem::transmute_copy(&radiisizes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radialgradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCoreProperties(::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturicollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePartUriCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturicollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageWriterOnFile(
                ::core::mem::transmute(&filename),
                ::core::mem::transmute_copy(&securityattributes),
                ::core::mem::transmute_copy(&flagsandattributes),
                ::core::mem::transmute_copy(&optimizemarkupsize),
                ::core::mem::transmute_copy(&interleaving),
                ::windows_core::from_raw_borrowed(&documentsequencepartname),
                ::windows_core::from_raw_borrowed(&coreproperties),
                ::windows_core::from_raw_borrowed(&packagethumbnail),
                ::windows_core::from_raw_borrowed(&documentsequenceprintticket),
                ::windows_core::from_raw_borrowed(&discardcontrolpartname),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageWriterOnStream(::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&coreproperties), ::windows_core::from_raw_borrowed(&packagethumbnail), ::windows_core::from_raw_borrowed(&documentsequenceprintticket), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePartUri(::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateReadOnlyStreamOnFile(::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackage: CreatePackage::<Identity, Impl, OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Identity, Impl, OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Identity, Impl, OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Identity, Impl, OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Identity, Impl, OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Identity, Impl, OFFSET>,
            CreatePartResources: CreatePartResources::<Identity, Impl, OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Identity, Impl, OFFSET>,
            CreateDocument: CreateDocument::<Identity, Impl, OFFSET>,
            CreatePageReference: CreatePageReference::<Identity, Impl, OFFSET>,
            CreatePage: CreatePage::<Identity, Impl, OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Identity, Impl, OFFSET>,
            CreateCanvas: CreateCanvas::<Identity, Impl, OFFSET>,
            CreateGlyphs: CreateGlyphs::<Identity, Impl, OFFSET>,
            CreatePath: CreatePath::<Identity, Impl, OFFSET>,
            CreateGeometry: CreateGeometry::<Identity, Impl, OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, Impl, OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Identity, Impl, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, Impl, OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Identity, Impl, OFFSET>,
            CreateImageResource: CreateImageResource::<Identity, Impl, OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            CreateGradientStop: CreateGradientStop::<Identity, Impl, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, Impl, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, Impl, OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Identity, Impl, OFFSET>,
            CreateDictionary: CreateDictionary::<Identity, Impl, OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Identity, Impl, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1_Impl: Sized + IXpsOMObjectFactory_Impl {
    fn GetDocumentTypeFromFile(&self, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn GetDocumentTypeFromStream(&self, xpsdocumentstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn ConvertHDPhotoToJpegXR(&self, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn ConvertJpegXRToHDPhoto(&self, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn CreatePackageWriterOnFile1(&self, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream1(&self, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackage1(&self) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromStream1(&self, stream: ::core::option::Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromFile1(&self, filename: &::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePage1(&self, pagedimensions: *const XPS_SIZE, language: &::windows_core::PCWSTR, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPage1>;
    fn CreatePageFromStream1(&self, pagemarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPage1>;
    fn CreateRemoteDictionaryResourceFromStream1(&self, dictionarymarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMObjectFactory1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory1_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>() -> IXpsOMObjectFactory1_Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentTypeFromFile(::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentTypeFromStream(::windows_core::from_raw_borrowed(&xpsdocumentstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertHDPhotoToJpegXR(::windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertJpegXRToHDPhoto(::windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageWriterOnFile1(
                ::core::mem::transmute(&filename),
                ::core::mem::transmute_copy(&securityattributes),
                ::core::mem::transmute_copy(&flagsandattributes),
                ::core::mem::transmute_copy(&optimizemarkupsize),
                ::core::mem::transmute_copy(&interleaving),
                ::windows_core::from_raw_borrowed(&documentsequencepartname),
                ::windows_core::from_raw_borrowed(&coreproperties),
                ::windows_core::from_raw_borrowed(&packagethumbnail),
                ::windows_core::from_raw_borrowed(&documentsequenceprintticket),
                ::windows_core::from_raw_borrowed(&discardcontrolpartname),
                ::core::mem::transmute_copy(&documenttype),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageWriterOnStream1(::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&coreproperties), ::windows_core::from_raw_borrowed(&packagethumbnail), ::windows_core::from_raw_borrowed(&documentsequenceprintticket), ::windows_core::from_raw_borrowed(&discardcontrolpartname), ::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackage1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageFromStream1(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePackageFromFile1(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePage1(::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute(&language), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePageFromStream1(::windows_core::from_raw_borrowed(&pagemarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRemoteDictionaryResourceFromStream1(::windows_core::from_raw_borrowed(&dictionarymarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMObjectFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Identity, Impl, OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Identity, Impl, OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Identity, Impl, OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Identity, Impl, OFFSET>,
            CreatePackage1: CreatePackage1::<Identity, Impl, OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Identity, Impl, OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Identity, Impl, OFFSET>,
            CreatePage1: CreatePage1::<Identity, Impl, OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory1 as ::windows_core::ComInterface>::IID || iid == &<IXpsOMObjectFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage_Impl: Sized {
    fn GetDocumentSequence(&self) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn SetDocumentSequence(&self, documentsequence: ::core::option::Option<&IXpsOMDocumentSequence>) -> ::windows_core::Result<()>;
    fn GetCoreProperties(&self) -> ::windows_core::Result<IXpsOMCoreProperties>;
    fn SetCoreProperties(&self, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>) -> ::windows_core::Result<()>;
    fn GetDiscardControlPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetDiscardControlPartName(&self, discardcontrolparturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn WriteToFile(&self, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteToStream(&self, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPackage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>() -> IXpsOMPackage_Vtbl {
        unsafe extern "system" fn GetDocumentSequence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentSequence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentSequence(::windows_core::from_raw_borrowed(&documentsequence)).into()
        }
        unsafe extern "system" fn GetCoreProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCoreProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCoreProperties(::windows_core::from_raw_borrowed(&coreproperties)).into()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDiscardControlPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(discardcontrolparturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscardControlPartName(::windows_core::from_raw_borrowed(&discardcontrolparturi)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnailResource(::windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn WriteToFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn WriteToStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToStream(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentSequence: GetDocumentSequence::<Identity, Impl, OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Identity, Impl, OFFSET>,
            GetCoreProperties: GetCoreProperties::<Identity, Impl, OFFSET>,
            SetCoreProperties: SetCoreProperties::<Identity, Impl, OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Identity, Impl, OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            WriteToFile: WriteToFile::<Identity, Impl, OFFSET>,
            WriteToStream: WriteToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1_Impl: Sized + IXpsOMPackage_Impl {
    fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(&self, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
    fn WriteToStream1(&self, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPackage1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>() -> IXpsOMPackage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToFile1(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        unsafe extern "system" fn WriteToStream1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToStream1(::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMPackage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            WriteToFile1: WriteToFile1::<Identity, Impl, OFFSET>,
            WriteToStream1: WriteToStream1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage1 as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPackage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageTarget_Impl: Sized {
    fn CreateXpsOMPackageWriter(&self, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPackageTarget {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: isize>() -> IXpsOMPackageTarget_Vtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateXpsOMPackageWriter(::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&documentsequenceprintticket), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageTarget as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter_Impl: Sized {
    fn StartNewDocument(&self, documentpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documentprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, documentstructure: ::core::option::Option<&IXpsOMDocumentStructureResource>, signatureblockresources: ::core::option::Option<&IXpsOMSignatureBlockResourceCollection>, restrictedfonts: ::core::option::Option<&IXpsOMPartUriCollection>) -> ::windows_core::Result<()>;
    fn AddPage(&self, page: ::core::option::Option<&IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::core::option::Option<&IXpsOMPartUriCollection>, storyfragments: ::core::option::Option<&IXpsOMStoryFragmentsResource>, pageprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, pagethumbnail: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn AddResource(&self, resource: ::core::option::Option<&IXpsOMResource>) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn IsClosed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPackageWriter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>() -> IXpsOMPackageWriter_Vtbl {
        unsafe extern "system" fn StartNewDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentpartname: *mut ::core::ffi::c_void, documentprintticket: *mut ::core::ffi::c_void, documentstructure: *mut ::core::ffi::c_void, signatureblockresources: *mut ::core::ffi::c_void, restrictedfonts: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartNewDocument(::windows_core::from_raw_borrowed(&documentpartname), ::windows_core::from_raw_borrowed(&documentprintticket), ::windows_core::from_raw_borrowed(&documentstructure), ::windows_core::from_raw_borrowed(&signatureblockresources), ::windows_core::from_raw_borrowed(&restrictedfonts)).into()
        }
        unsafe extern "system" fn AddPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut ::core::ffi::c_void, storyfragments: *mut ::core::ffi::c_void, pageprintticket: *mut ::core::ffi::c_void, pagethumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPage(::windows_core::from_raw_borrowed(&page), ::core::mem::transmute_copy(&advisorypagedimensions), ::windows_core::from_raw_borrowed(&discardableresourceparts), ::windows_core::from_raw_borrowed(&storyfragments), ::windows_core::from_raw_borrowed(&pageprintticket), ::windows_core::from_raw_borrowed(&pagethumbnail)).into()
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddResource(::windows_core::from_raw_borrowed(&resource)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn IsClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isclosed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartNewDocument: StartNewDocument::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3D_Impl: Sized + IXpsOMPackageWriter_Impl {
    fn AddModelTexture(&self, texturepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, texturedata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetModelPrintTicket(&self, printticketpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, printticketdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPackageWriter3D {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3D_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>() -> IXpsOMPackageWriter3D_Vtbl {
        unsafe extern "system" fn AddModelTexture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturepartname: *mut ::core::ffi::c_void, texturedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddModelTexture(::windows_core::from_raw_borrowed(&texturepartname), ::windows_core::from_raw_borrowed(&texturedata)).into()
        }
        unsafe extern "system" fn SetModelPrintTicket<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketpartname: *mut ::core::ffi::c_void, printticketdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModelPrintTicket(::windows_core::from_raw_borrowed(&printticketpartname), ::windows_core::from_raw_borrowed(&printticketdata)).into()
        }
        Self {
            base__: IXpsOMPackageWriter_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddModelTexture: AddModelTexture::<Identity, Impl, OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPackageWriter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage_Impl: Sized + IXpsOMPart_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPageReference>;
    fn GetVisuals(&self) -> ::windows_core::Result<IXpsOMVisualCollection>;
    fn GetPageDimensions(&self) -> ::windows_core::Result<XPS_SIZE>;
    fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetContentBox(&self) -> ::windows_core::Result<XPS_RECT>;
    fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetBleedBox(&self) -> ::windows_core::Result<XPS_RECT>;
    fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(&self, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetName(&self, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Write(&self, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMPage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>() -> IXpsOMPage_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visuals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagedimensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetContentBox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentBox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentBox(::core::mem::transmute_copy(&contentbox)).into()
        }
        unsafe extern "system" fn GetBleedBox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBleedBox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bleedbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBleedBox(::core::mem::transmute_copy(&bleedbox)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguage(::core::mem::transmute(&language)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ishyperlinktarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlinktarget)).into()
        }
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionaryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictionaryLocal(::windows_core::from_raw_borrowed(&resourcedictionary)).into()
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionaryResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictionaryResource(::windows_core::from_raw_borrowed(&remotedictionaryresource)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateUnusedLookupKey(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetPageDimensions: GetPageDimensions::<Identity, Impl, OFFSET>,
            SetPageDimensions: SetPageDimensions::<Identity, Impl, OFFSET>,
            GetContentBox: GetContentBox::<Identity, Impl, OFFSET>,
            SetContentBox: SetContentBox::<Identity, Impl, OFFSET>,
            GetBleedBox: GetBleedBox::<Identity, Impl, OFFSET>,
            SetBleedBox: SetBleedBox::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPage as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1_Impl: Sized + IXpsOMPage_Impl {
    fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPage1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>() -> IXpsOMPage1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write1(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMPage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPage1 as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMPageReference_Impl: Sized {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetPage(&self) -> ::windows_core::Result<IXpsOMPage>;
    fn SetPage(&self, page: ::core::option::Option<&IXpsOMPage>) -> ::windows_core::Result<()>;
    fn DiscardPage(&self) -> ::windows_core::Result<()>;
    fn IsPageLoaded(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAdvisoryPageDimensions(&self) -> ::windows_core::Result<XPS_SIZE>;
    fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetStoryFragmentsResource(&self) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn SetStoryFragmentsResource(&self, storyfragmentsresource: ::core::option::Option<&IXpsOMStoryFragmentsResource>) -> ::windows_core::Result<()>;
    fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn CollectLinkTargets(&self) -> ::windows_core::Result<IXpsOMNameCollection>;
    fn CollectPartResources(&self) -> ::windows_core::Result<IXpsOMPartResources>;
    fn HasRestrictedFonts(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMPageReference>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IXpsOMPageReference {}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMPageReference_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>() -> IXpsOMPageReference_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPage(::windows_core::from_raw_borrowed(&page)).into()
        }
        unsafe extern "system" fn DiscardPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardPage().into()
        }
        unsafe extern "system" fn IsPageLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPageLoaded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispageloaded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdvisoryPageDimensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagedimensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdvisoryPageDimensions(::core::mem::transmute_copy(&pagedimensions)).into()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStoryFragmentsResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyfragmentsresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStoryFragmentsResource(::windows_core::from_raw_borrowed(&storyfragmentsresource)).into()
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrintTicketResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrintTicketResource(::windows_core::from_raw_borrowed(&printticketresource)).into()
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailResource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThumbnailResource(::windows_core::from_raw_borrowed(&imageresource)).into()
        }
        unsafe extern "system" fn CollectLinkTargets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linktargets: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectLinkTargets() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linktargets, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectPartResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasRestrictedFonts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictedfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPage: GetPage::<Identity, Impl, OFFSET>,
            SetPage: SetPage::<Identity, Impl, OFFSET>,
            DiscardPage: DiscardPage::<Identity, Impl, OFFSET>,
            IsPageLoaded: IsPageLoaded::<Identity, Impl, OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Identity, Impl, OFFSET>,
            CollectPartResources: CollectPartResources::<Identity, Impl, OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReference as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMPageReferenceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMPageReference>;
    fn InsertAt(&self, index: u32, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
    fn Append(&self, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMPageReferenceCollection {}
impl IXpsOMPageReferenceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>() -> IXpsOMPageReferenceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&pagereference)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&pagereference)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReferenceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPart_Impl: Sized {
    fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(&self, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPart {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPart_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>() -> IXpsOMPart_Vtbl {
        unsafe extern "system" fn GetPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPartName(::windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            SetPartName: SetPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMPartResources_Impl: Sized {
    fn GetFontResources(&self) -> ::windows_core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(&self) -> ::windows_core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(&self) -> ::windows_core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl ::windows_core::RuntimeName for IXpsOMPartResources {}
impl IXpsOMPartResources_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>() -> IXpsOMPartResources_Vtbl {
        unsafe extern "system" fn GetFontResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFontResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColorProfileResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRemoteDictionaryResources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontResources: GetFontResources::<Identity, Impl, OFFSET>,
            GetImageResources: GetImageResources::<Identity, Impl, OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Identity, Impl, OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPartResources as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartUriCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(&self, index: u32, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn Append(&self, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPartUriCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartUriCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>() -> IXpsOMPartUriCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&parturi)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&parturi)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPartUriCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMPath_Impl: Sized + IXpsOMVisual_Impl {
    fn GetGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn SetGeometryLocal(&self, geometry: ::core::option::Option<&IXpsOMGeometry>) -> ::windows_core::Result<()>;
    fn GetGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetGeometryLookup(&self, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSnapsToPixels(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSnapsToPixels(&self, snapstopixels: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetStrokeBrush(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetStrokeBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetStrokeBrushLocal(&self, brush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetStrokeBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetStrokeBrushLookup(&self, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStrokeDashes(&self) -> ::windows_core::Result<IXpsOMDashCollection>;
    fn GetStrokeDashCap(&self) -> ::windows_core::Result<XPS_DASH_CAP>;
    fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeDashOffset(&self) -> ::windows_core::Result<f32>;
    fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows_core::Result<()>;
    fn GetStrokeStartLineCap(&self) -> ::windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeEndLineCap(&self) -> ::windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeLineJoin(&self) -> ::windows_core::Result<XPS_LINE_JOIN>;
    fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::Result<()>;
    fn GetStrokeMiterLimit(&self) -> ::windows_core::Result<f32>;
    fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows_core::Result<()>;
    fn GetStrokeThickness(&self) -> ::windows_core::Result<f32>;
    fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows_core::Result<()>;
    fn GetFillBrush(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, brush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMPath>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPath {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMPath_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>() -> IXpsOMPath_Vtbl {
        unsafe extern "system" fn GetGeometry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGeometryLocal(::windows_core::from_raw_borrowed(&geometry)).into()
        }
        unsafe extern "system" fn GetGeometryLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGeometryLookup(::core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessibilityShortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessibilityShortDescription(::core::mem::transmute(&shortdescription)).into()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessibilityLongDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(longdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccessibilityLongDescription(::core::mem::transmute(&longdescription)).into()
        }
        unsafe extern "system" fn GetSnapsToPixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSnapsToPixels() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapstopixels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSnapsToPixels(::core::mem::transmute_copy(&snapstopixels)).into()
        }
        unsafe extern "system" fn GetStrokeBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeBrush() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeBrushLocal(::windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeBrushLookup(::core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn GetStrokeDashes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeDashes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeDashCap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashcap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeDashCap(::core::mem::transmute_copy(&strokedashcap)).into()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeDashOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeDashOffset(::core::mem::transmute_copy(&strokedashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeStartLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokestartlinecap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeStartLineCap(::core::mem::transmute_copy(&strokestartlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeEndLineCap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokeendlinecap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeEndLineCap(::core::mem::transmute_copy(&strokeendlinecap)).into()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeLineJoin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokelinejoin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeLineJoin(::core::mem::transmute_copy(&strokelinejoin)).into()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeMiterLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokemiterlimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeMiterLimit(::core::mem::transmute_copy(&strokemiterlimit)).into()
        }
        unsafe extern "system" fn GetStrokeThickness<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStrokeThickness() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokethickness, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrokeThickness(::core::mem::transmute_copy(&strokethickness)).into()
        }
        unsafe extern "system" fn GetFillBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrush() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFillBrushLocal(::windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFillBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFillBrushLookup(::core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, Impl, OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Identity, Impl, OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Identity, Impl, OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Identity, Impl, OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Identity, Impl, OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Identity, Impl, OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Identity, Impl, OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Identity, Impl, OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Identity, Impl, OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Identity, Impl, OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Identity, Impl, OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Identity, Impl, OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Identity, Impl, OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Identity, Impl, OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Identity, Impl, OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPath as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMVisual as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMPrintTicketResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>() -> IXpsOMPrintTicketResource_Vtbl {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMRadialGradientBrush_Impl: Sized + IXpsOMGradientBrush_Impl {
    fn GetCenter(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetCenter(&self, center: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetRadiiSizes(&self) -> ::windows_core::Result<XPS_SIZE>;
    fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetGradientOrigin(&self) -> ::windows_core::Result<XPS_POINT>;
    fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMRadialGradientBrush>;
}
impl ::windows_core::RuntimeName for IXpsOMRadialGradientBrush {}
impl IXpsOMRadialGradientBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>() -> IXpsOMRadialGradientBrush_Vtbl {
        unsafe extern "system" fn GetCenter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCenter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(center, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCenter(::core::mem::transmute_copy(&center)).into()
        }
        unsafe extern "system" fn GetRadiiSizes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRadiiSizes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radiisizes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRadiiSizes(::core::mem::transmute_copy(&radiisizes)).into()
        }
        unsafe extern "system" fn GetGradientOrigin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGradientOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(origin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGradientOrigin(::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radialgradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCenter: GetCenter::<Identity, Impl, OFFSET>,
            SetCenter: SetCenter::<Identity, Impl, OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Identity, Impl, OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Identity, Impl, OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Identity, Impl, OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMRadialGradientBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMGradientBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionary(&self, dictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMRemoteDictionaryResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource_Vtbl {
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictionary(::windows_core::from_raw_borrowed(&dictionary)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            SetDictionary: SetDictionary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1_Impl: Sized + IXpsOMRemoteDictionaryResource_Impl {
    fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMRemoteDictionaryResource1 {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1_Vtbl {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write1(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&documenttype)).into()
        }
        Self {
            base__: IXpsOMRemoteDictionaryResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMRemoteDictionaryResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(&self, index: u32, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Append(&self, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMRemoteDictionaryResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByPartName(::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResourceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMResource_Impl: Sized + IXpsOMPart_Impl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMResource_Impl, const OFFSET: isize>() -> IXpsOMResource_Vtbl {
        Self { base__: IXpsOMPart_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMShareable_Impl: Sized {
    fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE>;
}
impl ::windows_core::RuntimeName for IXpsOMShareable {}
impl IXpsOMShareable_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>() -> IXpsOMShareable_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMSignatureBlockResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>() -> IXpsOMSignatureBlockResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(&self, index: u32, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn Append(&self, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMSignatureBlockResourceCollection {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&signatureblockresource)).into()
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByPartName(::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResourceCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMSolidColorBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMSolidColorBrush>;
}
impl ::windows_core::RuntimeName for IXpsOMSolidColorBrush {}
impl IXpsOMSolidColorBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>() -> IXpsOMSolidColorBrush_Vtbl {
        unsafe extern "system" fn GetColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into()
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColor(::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(solidcolorbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMSolidColorBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResource_Impl: Sized + IXpsOMResource_Impl {
    fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPageReference>;
    fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMStoryFragmentsResource {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>() -> IXpsOMStoryFragmentsResource_Vtbl {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContent(::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into()
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMStoryFragmentsResource as ::windows_core::ComInterface>::IID || iid == &<IXpsOMPart as ::windows_core::ComInterface>::IID || iid == &<IXpsOMResource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMThumbnailGenerator_Impl: Sized {
    fn GenerateThumbnail(&self, page: ::core::option::Option<&IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMThumbnailGenerator {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMThumbnailGenerator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>() -> IXpsOMThumbnailGenerator_Vtbl {
        unsafe extern "system" fn GenerateThumbnail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateThumbnail(::windows_core::from_raw_borrowed(&page), ::core::mem::transmute_copy(&thumbnailtype), ::core::mem::transmute_copy(&thumbnailsize), ::windows_core::from_raw_borrowed(&imageresourcepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GenerateThumbnail: GenerateThumbnail::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMThumbnailGenerator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMTileBrush_Impl: Sized + IXpsOMBrush_Impl {
    fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetViewbox(&self) -> ::windows_core::Result<XPS_RECT>;
    fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetViewport(&self) -> ::windows_core::Result<XPS_RECT>;
    fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetTileMode(&self) -> ::windows_core::Result<XPS_TILE_MODE>;
    fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMTileBrush {}
impl IXpsOMTileBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>() -> IXpsOMTileBrush_Vtbl {
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLocal(::windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetViewbox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewbox() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewbox(::core::mem::transmute_copy(&viewbox)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewport(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn GetTileMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTileMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tilemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTileMode(::core::mem::transmute_copy(&tilemode)).into()
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetViewbox: GetViewbox::<Identity, Impl, OFFSET>,
            SetViewbox: SetViewbox::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            SetViewport: SetViewport::<Identity, Impl, OFFSET>,
            GetTileMode: GetTileMode::<Identity, Impl, OFFSET>,
            SetTileMode: SetTileMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMTileBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMVisual_Impl: Sized + IXpsOMShareable_Impl {
    fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, matrixtransform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetClipGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetClipGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry>;
    fn SetClipGeometryLocal(&self, clipgeometry: ::core::option::Option<&IXpsOMGeometry>) -> ::windows_core::Result<()>;
    fn GetClipGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetClipGeometryLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetOpacity(&self) -> ::windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()>;
    fn GetOpacityMaskBrush(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetOpacityMaskBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetOpacityMaskBrushLocal(&self, opacitymaskbrush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetOpacityMaskBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetOpacityMaskBrushLookup(&self, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetName(&self, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlink: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetHyperlinkNavigateUri(&self) -> ::windows_core::Result<super::super::System::Com::IUri>;
    fn SetHyperlinkNavigateUri(&self, hyperlinkuri: ::core::option::Option<&super::super::System::Com::IUri>) -> ::windows_core::Result<()>;
    fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(&self, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsOMVisual {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMVisual_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>() -> IXpsOMVisual_Vtbl {
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLocal(::windows_core::from_raw_borrowed(&matrixtransform)).into()
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetClipGeometry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipGeometry() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clipgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipGeometryLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clipgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipGeometryLocal(::windows_core::from_raw_borrowed(&clipgeometry)).into()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClipGeometryLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClipGeometryLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetOpacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpacity(::core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpacityMaskBrush() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacitymaskbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpacityMaskBrushLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacitymaskbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpacityMaskBrushLocal(::windows_core::from_raw_borrowed(&opacitymaskbrush)).into()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOpacityMaskBrushLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpacityMaskBrushLookup(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIsHyperlinkTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ishyperlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlink)).into()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHyperlinkNavigateUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hyperlinkuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHyperlinkNavigateUri(::windows_core::from_raw_borrowed(&hyperlinkuri)).into()
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguage(::core::mem::transmute(&language)).into()
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetClipGeometry: GetClipGeometry::<Identity, Impl, OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Identity, Impl, OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Identity, Impl, OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Identity, Impl, OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Identity, Impl, OFFSET>,
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMVisual as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMVisualBrush_Impl: Sized + IXpsOMTileBrush_Impl {
    fn GetVisual(&self) -> ::windows_core::Result<IXpsOMVisual>;
    fn GetVisualLocal(&self) -> ::windows_core::Result<IXpsOMVisual>;
    fn SetVisualLocal(&self, visual: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn GetVisualLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetVisualLookup(&self, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IXpsOMVisualBrush>;
}
impl ::windows_core::RuntimeName for IXpsOMVisualBrush {}
impl IXpsOMVisualBrush_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>() -> IXpsOMVisualBrush_Vtbl {
        unsafe extern "system" fn GetVisual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisual() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisualLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVisualLocal(::windows_core::from_raw_borrowed(&visual)).into()
        }
        unsafe extern "system" fn GetVisualLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisualLookup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVisualLookup(::core::mem::transmute(&lookup)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visualbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVisual: GetVisual::<Identity, Impl, OFFSET>,
            GetVisualLocal: GetVisualLocal::<Identity, Impl, OFFSET>,
            SetVisualLocal: SetVisualLocal::<Identity, Impl, OFFSET>,
            GetVisualLookup: GetVisualLookup::<Identity, Impl, OFFSET>,
            SetVisualLookup: SetVisualLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMShareable as ::windows_core::ComInterface>::IID || iid == &<IXpsOMBrush as ::windows_core::ComInterface>::IID || iid == &<IXpsOMTileBrush as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsOMVisualCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMVisual>;
    fn InsertAt(&self, index: u32, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn Append(&self, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsOMVisualCollection {}
impl IXpsOMVisualCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>() -> IXpsOMVisualCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&object)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignature_Impl: Sized {
    fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetCertificateEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator>;
    fn GetSigningTime(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSigningTimeFormat(&self) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn GetSignaturePartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<XPS_SIGNATURE_STATUS>;
    fn GetPolicy(&self) -> ::windows_core::Result<XPS_SIGN_POLICY>;
    fn GetCustomObjectEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn SetSignatureXml(&self, signaturexml: *const u8, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsSignature {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignature_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>() -> IXpsSignature_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignatureValue(::core::mem::transmute_copy(&signaturehashvalue), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertificateEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigdatetimestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Verify(::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomObjectEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomReferenceEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetSignatureXml<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, Impl, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, Impl, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            Verify: Verify::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, Impl, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, Impl, OFFSET>,
            SetSignatureXml: SetSignatureXml::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignature as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureBlock_Impl: Sized {
    fn GetRequests(&self) -> ::windows_core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(&self) -> ::windows_core::Result<u32>;
    fn GetDocumentName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(&self, requestid: &::windows_core::PCWSTR) -> ::windows_core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsSignatureBlock {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureBlock_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>() -> IXpsSignatureBlock_Vtbl {
        unsafe extern "system" fn GetRequests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requests: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequests() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixeddocumentindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixeddocumentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::PCWSTR, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRequest(::core::mem::transmute(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturerequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequests: GetRequests::<Identity, Impl, OFFSET>,
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Identity, Impl, OFFSET>,
            GetDocumentName: GetDocumentName::<Identity, Impl, OFFSET>,
            CreateRequest: CreateRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlock as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsSignatureBlockCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignatureBlock>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsSignatureBlockCollection {}
impl IXpsSignatureBlockCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>() -> IXpsSignatureBlockCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlockCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsSignatureCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignature>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsSignatureCollection {}
impl IXpsSignatureCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>() -> IXpsSignatureCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManager_Impl: Sized {
    fn LoadPackageFile(&self, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LoadPackageStream(&self, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Sign(&self, signoptions: ::core::option::Option<&IXpsSigningOptions>, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<IXpsSignature>;
    fn GetSignatureOriginPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetSignatures(&self) -> ::windows_core::Result<IXpsSignatureCollection>;
    fn AddSignatureBlock(&self, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, fixeddocumentindex: u32) -> ::windows_core::Result<IXpsSignatureBlock>;
    fn GetSignatureBlocks(&self) -> ::windows_core::Result<IXpsSignatureBlockCollection>;
    fn CreateSigningOptions(&self) -> ::windows_core::Result<IXpsSigningOptions>;
    fn SavePackageToFile(&self, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::Result<()>;
    fn SavePackageToStream(&self, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsSignatureManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>() -> IXpsSignatureManager_Vtbl {
        unsafe extern "system" fn LoadPackageFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadPackageFile(::core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn LoadPackageStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadPackageStream(::windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn Sign<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signoptions: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sign(::windows_core::from_raw_borrowed(&signoptions), ::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureOriginPartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureoriginpartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureOriginPartName(::windows_core::from_raw_borrowed(&signatureoriginpartname)).into()
        }
        unsafe extern "system" fn GetSignatures<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatures() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddSignatureBlock(::windows_core::from_raw_borrowed(&partname), ::core::mem::transmute_copy(&fixeddocumentindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSigningOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SavePackageToFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes)).into()
        }
        unsafe extern "system" fn SavePackageToStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SavePackageToStream(::windows_core::from_raw_borrowed(&stream)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadPackageFile: LoadPackageFile::<Identity, Impl, OFFSET>,
            LoadPackageStream: LoadPackageStream::<Identity, Impl, OFFSET>,
            Sign: Sign::<Identity, Impl, OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            GetSignatures: GetSignatures::<Identity, Impl, OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Identity, Impl, OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Identity, Impl, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, Impl, OFFSET>,
            SavePackageToFile: SavePackageToFile::<Identity, Impl, OFFSET>,
            SavePackageToStream: SavePackageToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureRequest_Impl: Sized {
    fn GetIntent(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIntent(&self, intent: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRequestedSigner(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRequestedSigner(&self, signername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRequestSignByDate(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRequestSignByDate(&self, datestring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSigningLocale(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSigningLocale(&self, place: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows_core::Result<()>;
    fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn GetRequestId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignature(&self) -> ::windows_core::Result<IXpsSignature>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsSignatureRequest {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>() -> IXpsSignatureRequest_Vtbl {
        unsafe extern "system" fn GetIntent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIntent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntent(::core::mem::transmute(&intent)).into()
        }
        unsafe extern "system" fn GetRequestedSigner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequestedSigner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestedSigner(::core::mem::transmute(&signername)).into()
        }
        unsafe extern "system" fn GetRequestSignByDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequestSignByDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestSignByDate(::core::mem::transmute(&datestring)).into()
        }
        unsafe extern "system" fn GetSigningLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(place, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSigningLocale(::core::mem::transmute(&place)).into()
        }
        unsafe extern "system" fn GetSpotLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut ::core::ffi::c_void, x: *mut f32, y: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&pagepartname), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetSpotLocation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn GetRequestId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequestId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIntent: GetIntent::<Identity, Impl, OFFSET>,
            SetIntent: SetIntent::<Identity, Impl, OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Identity, Impl, OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Identity, Impl, OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Identity, Impl, OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Identity, Impl, OFFSET>,
            GetSigningLocale: GetSigningLocale::<Identity, Impl, OFFSET>,
            SetSigningLocale: SetSigningLocale::<Identity, Impl, OFFSET>,
            GetSpotLocation: GetSpotLocation::<Identity, Impl, OFFSET>,
            SetSpotLocation: SetSpotLocation::<Identity, Impl, OFFSET>,
            GetRequestId: GetRequestId::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequest as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"implement\"`*"]
pub trait IXpsSignatureRequestCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignatureRequest>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXpsSignatureRequestCollection {}
impl IXpsSignatureRequestCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>() -> IXpsSignatureRequestCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturerequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequestCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSigningOptions_Impl: Sized {
    fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignatureMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDigestMethod(&self, digestmethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignaturePartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetPolicy(&self) -> ::windows_core::Result<XPS_SIGN_POLICY>;
    fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows_core::Result<()>;
    fn GetSigningTimeFormat(&self) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::Result<()>;
    fn GetCustomObjects(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet>;
    fn GetCustomReferences(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateSet>;
    fn GetFlags(&self) -> ::windows_core::Result<XPS_SIGN_FLAGS>;
    fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXpsSigningOptions {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSigningOptions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>() -> IXpsSigningOptions_Vtbl {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureId(::core::mem::transmute(&signatureid)).into()
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignatureMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignatureMethod(::core::mem::transmute(&signaturemethod)).into()
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDigestMethod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDigestMethod(::core::mem::transmute(&digestmethod)).into()
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignaturePartName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignaturePartName(::windows_core::from_raw_borrowed(&signaturepartname)).into()
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSigningTimeFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSigningTimeFormat(::core::mem::transmute_copy(&timeformat)).into()
        }
        unsafe extern "system" fn GetCustomObjects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomReferences() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertificateSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            SetDigestMethod: SetDigestMethod::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            SetPolicy: SetPolicy::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetCustomObjects: GetCustomObjects::<Identity, Impl, OFFSET>,
            GetCustomReferences: GetCustomReferences::<Identity, Impl, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXpsSigningOptions as ::windows_core::ComInterface>::IID
    }
}
