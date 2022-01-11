#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsDocumentPackageTargetImpl: Sized {
    fn GetXpsOMPackageWriter();
    fn GetXpsOMFactory();
    fn GetXpsType();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsDocumentPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTargetVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetXpsType<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetXpsOMPackageWriter::<Impl, IMPL_OFFSET>, GetXpsOMFactory::<Impl, IMPL_OFFSET>, GetXpsType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3DImpl: Sized {
    fn GetXpsOMPackageWriter3D();
    fn GetXpsOMFactory();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsDocumentPackageTarget3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsDocumentPackageTarget3DVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetXpsOMPackageWriter3D::<Impl, IMPL_OFFSET>, GetXpsOMFactory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget3D as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMBrushImpl: Sized + IXpsOMShareableImpl {
    fn GetOpacity();
    fn SetOpacity();
}
impl IXpsOMBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMBrushVtbl {
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetOpacity::<Impl, IMPL_OFFSET>, SetOpacity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMCanvasImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetVisuals();
    fn GetUseAliasedEdgeMode();
    fn SetUseAliasedEdgeMode();
    fn GetAccessibilityShortDescription();
    fn SetAccessibilityShortDescription();
    fn GetAccessibilityLongDescription();
    fn SetAccessibilityLongDescription();
    fn GetDictionary();
    fn GetDictionaryLocal();
    fn SetDictionaryLocal();
    fn GetDictionaryResource();
    fn SetDictionaryResource();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMCanvasVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCanvasImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCanvasVtbl {
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCanvasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetClipGeometry::<Impl, IMPL_OFFSET>,
            GetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            SetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            GetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            SetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrush::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            SetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetVisuals::<Impl, IMPL_OFFSET>,
            GetUseAliasedEdgeMode::<Impl, IMPL_OFFSET>,
            SetUseAliasedEdgeMode::<Impl, IMPL_OFFSET>,
            GetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            GetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            GetDictionary::<Impl, IMPL_OFFSET>,
            GetDictionaryLocal::<Impl, IMPL_OFFSET>,
            SetDictionaryLocal::<Impl, IMPL_OFFSET>,
            GetDictionaryResource::<Impl, IMPL_OFFSET>,
            SetDictionaryResource::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMCanvas as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMColorProfileResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMColorProfileResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMColorProfileResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMColorProfileResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, GetByPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsOMCorePropertiesImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetCategory();
    fn SetCategory();
    fn GetContentStatus();
    fn SetContentStatus();
    fn GetContentType();
    fn SetContentType();
    fn GetCreated();
    fn SetCreated();
    fn GetCreator();
    fn SetCreator();
    fn GetDescription();
    fn SetDescription();
    fn GetIdentifier();
    fn SetIdentifier();
    fn GetKeywords();
    fn SetKeywords();
    fn GetLanguage();
    fn SetLanguage();
    fn GetLastModifiedBy();
    fn SetLastModifiedBy();
    fn GetLastPrinted();
    fn SetLastPrinted();
    fn GetModified();
    fn SetModified();
    fn GetRevision();
    fn SetRevision();
    fn GetSubject();
    fn SetSubject();
    fn GetTitle();
    fn SetTitle();
    fn GetVersion();
    fn SetVersion();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsOMCorePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMCorePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMCorePropertiesVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifier: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, revision: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPartName::<Impl, IMPL_OFFSET>,
            SetPartName::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetCategory::<Impl, IMPL_OFFSET>,
            SetCategory::<Impl, IMPL_OFFSET>,
            GetContentStatus::<Impl, IMPL_OFFSET>,
            SetContentStatus::<Impl, IMPL_OFFSET>,
            GetContentType::<Impl, IMPL_OFFSET>,
            SetContentType::<Impl, IMPL_OFFSET>,
            GetCreated::<Impl, IMPL_OFFSET>,
            SetCreated::<Impl, IMPL_OFFSET>,
            GetCreator::<Impl, IMPL_OFFSET>,
            SetCreator::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetIdentifier::<Impl, IMPL_OFFSET>,
            SetIdentifier::<Impl, IMPL_OFFSET>,
            GetKeywords::<Impl, IMPL_OFFSET>,
            SetKeywords::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetLastModifiedBy::<Impl, IMPL_OFFSET>,
            SetLastModifiedBy::<Impl, IMPL_OFFSET>,
            GetLastPrinted::<Impl, IMPL_OFFSET>,
            SetLastPrinted::<Impl, IMPL_OFFSET>,
            GetModified::<Impl, IMPL_OFFSET>,
            SetModified::<Impl, IMPL_OFFSET>,
            GetRevision::<Impl, IMPL_OFFSET>,
            SetRevision::<Impl, IMPL_OFFSET>,
            GetSubject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            GetTitle::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMCoreProperties as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMDashCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMDashCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDashCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDashCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDashCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDashCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMDictionaryImpl: Sized {
    fn GetOwner();
    fn GetCount();
    fn GetAt();
    fn GetByKey();
    fn GetIndex();
    fn Append();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDictionaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDictionaryVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByKey<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIndex<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetByKey::<Impl, IMPL_OFFSET>, GetIndex::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDictionary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMDocumentImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetPageReferences();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
    fn GetDocumentStructureResource();
    fn SetDocumentStructureResource();
    fn GetSignatureBlockResources();
    fn Clone();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageReferences<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureBlockResources<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPartName::<Impl, IMPL_OFFSET>,
            SetPartName::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetPageReferences::<Impl, IMPL_OFFSET>,
            GetPrintTicketResource::<Impl, IMPL_OFFSET>,
            SetPrintTicketResource::<Impl, IMPL_OFFSET>,
            GetDocumentStructureResource::<Impl, IMPL_OFFSET>,
            SetDocumentStructureResource::<Impl, IMPL_OFFSET>,
            GetSignatureBlockResources::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocument as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMDocumentCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMDocumentCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMDocumentSequenceImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetDocuments();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMDocumentSequenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentSequenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentSequenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocuments<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetDocuments::<Impl, IMPL_OFFSET>, GetPrintTicketResource::<Impl, IMPL_OFFSET>, SetPrintTicketResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentSequence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMDocumentStructureResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMDocumentStructureResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMDocumentStructureResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetEmbeddingOption();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEmbeddingOption<Impl: IXpsOMFontResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>, GetEmbeddingOption::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMFontResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMFontResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn SetAt();
    fn InsertAt();
    fn Append();
    fn RemoveAt();
    fn GetByPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMFontResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMFontResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMFontResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, GetByPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMFontResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryImpl: Sized + IXpsOMShareableImpl {
    fn GetFigures();
    fn GetFillRule();
    fn SetFillRule();
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGeometryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryVtbl {
        unsafe extern "system" fn GetFigures<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetFigures::<Impl, IMPL_OFFSET>,
            GetFillRule::<Impl, IMPL_OFFSET>,
            SetFillRule::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryFigureImpl: Sized {
    fn GetOwner();
    fn GetSegmentData();
    fn GetSegmentTypes();
    fn GetSegmentStrokes();
    fn SetSegments();
    fn GetStartPoint();
    fn SetStartPoint();
    fn GetIsClosed();
    fn SetIsClosed();
    fn GetIsFilled();
    fn SetIsFilled();
    fn GetSegmentCount();
    fn GetSegmentDataCount();
    fn GetSegmentStrokePattern();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGeometryFigureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigureVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentData<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentTypes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentStrokes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSegments<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryFigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetSegmentData::<Impl, IMPL_OFFSET>,
            GetSegmentTypes::<Impl, IMPL_OFFSET>,
            GetSegmentStrokes::<Impl, IMPL_OFFSET>,
            SetSegments::<Impl, IMPL_OFFSET>,
            GetStartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint::<Impl, IMPL_OFFSET>,
            GetIsClosed::<Impl, IMPL_OFFSET>,
            SetIsClosed::<Impl, IMPL_OFFSET>,
            GetIsFilled::<Impl, IMPL_OFFSET>,
            SetIsFilled::<Impl, IMPL_OFFSET>,
            GetSegmentCount::<Impl, IMPL_OFFSET>,
            GetSegmentDataCount::<Impl, IMPL_OFFSET>,
            GetSegmentStrokePattern::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigure as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGeometryFigureCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMGeometryFigureCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGeometryFigureCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGeometryFigureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigureCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMGlyphsImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetUnicodeString();
    fn GetGlyphIndexCount();
    fn GetGlyphIndices();
    fn GetGlyphMappingCount();
    fn GetGlyphMappings();
    fn GetProhibitedCaretStopCount();
    fn GetProhibitedCaretStops();
    fn GetBidiLevel();
    fn GetIsSideways();
    fn GetDeviceFontName();
    fn GetStyleSimulations();
    fn SetStyleSimulations();
    fn GetOrigin();
    fn SetOrigin();
    fn GetFontRenderingEmSize();
    fn SetFontRenderingEmSize();
    fn GetFontResource();
    fn SetFontResource();
    fn GetFontFaceIndex();
    fn SetFontFaceIndex();
    fn GetFillBrush();
    fn GetFillBrushLocal();
    fn SetFillBrushLocal();
    fn GetFillBrushLookup();
    fn SetFillBrushLookup();
    fn GetGlyphsEditor();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMGlyphsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphsVtbl {
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphsEditor<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGlyphsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetClipGeometry::<Impl, IMPL_OFFSET>,
            GetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            SetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            GetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            SetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrush::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            SetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetUnicodeString::<Impl, IMPL_OFFSET>,
            GetGlyphIndexCount::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            GetGlyphMappingCount::<Impl, IMPL_OFFSET>,
            GetGlyphMappings::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStopCount::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            GetBidiLevel::<Impl, IMPL_OFFSET>,
            GetIsSideways::<Impl, IMPL_OFFSET>,
            GetDeviceFontName::<Impl, IMPL_OFFSET>,
            GetStyleSimulations::<Impl, IMPL_OFFSET>,
            SetStyleSimulations::<Impl, IMPL_OFFSET>,
            GetOrigin::<Impl, IMPL_OFFSET>,
            SetOrigin::<Impl, IMPL_OFFSET>,
            GetFontRenderingEmSize::<Impl, IMPL_OFFSET>,
            SetFontRenderingEmSize::<Impl, IMPL_OFFSET>,
            GetFontResource::<Impl, IMPL_OFFSET>,
            SetFontResource::<Impl, IMPL_OFFSET>,
            GetFontFaceIndex::<Impl, IMPL_OFFSET>,
            SetFontFaceIndex::<Impl, IMPL_OFFSET>,
            GetFillBrush::<Impl, IMPL_OFFSET>,
            GetFillBrushLocal::<Impl, IMPL_OFFSET>,
            SetFillBrushLocal::<Impl, IMPL_OFFSET>,
            GetFillBrushLookup::<Impl, IMPL_OFFSET>,
            SetFillBrushLookup::<Impl, IMPL_OFFSET>,
            GetGlyphsEditor::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGlyphs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGlyphsEditorImpl: Sized {
    fn ApplyEdits();
    fn GetUnicodeString();
    fn SetUnicodeString();
    fn GetGlyphIndexCount();
    fn GetGlyphIndices();
    fn SetGlyphIndices();
    fn GetGlyphMappingCount();
    fn GetGlyphMappings();
    fn SetGlyphMappings();
    fn GetProhibitedCaretStopCount();
    fn GetProhibitedCaretStops();
    fn SetProhibitedCaretStops();
    fn GetBidiLevel();
    fn SetBidiLevel();
    fn GetIsSideways();
    fn SetIsSideways();
    fn GetDeviceFontName();
    fn SetDeviceFontName();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGlyphsEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGlyphsEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGlyphsEditorVtbl {
        unsafe extern "system" fn ApplyEdits<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ApplyEdits::<Impl, IMPL_OFFSET>,
            GetUnicodeString::<Impl, IMPL_OFFSET>,
            SetUnicodeString::<Impl, IMPL_OFFSET>,
            GetGlyphIndexCount::<Impl, IMPL_OFFSET>,
            GetGlyphIndices::<Impl, IMPL_OFFSET>,
            SetGlyphIndices::<Impl, IMPL_OFFSET>,
            GetGlyphMappingCount::<Impl, IMPL_OFFSET>,
            GetGlyphMappings::<Impl, IMPL_OFFSET>,
            SetGlyphMappings::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStopCount::<Impl, IMPL_OFFSET>,
            GetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            SetProhibitedCaretStops::<Impl, IMPL_OFFSET>,
            GetBidiLevel::<Impl, IMPL_OFFSET>,
            SetBidiLevel::<Impl, IMPL_OFFSET>,
            GetIsSideways::<Impl, IMPL_OFFSET>,
            SetIsSideways::<Impl, IMPL_OFFSET>,
            GetDeviceFontName::<Impl, IMPL_OFFSET>,
            SetDeviceFontName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGlyphsEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGradientBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetGradientStops();
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetSpreadMethod();
    fn SetSpreadMethod();
    fn GetColorInterpolationMode();
    fn SetColorInterpolationMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientBrushVtbl {
        unsafe extern "system" fn GetGradientStops<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetGradientStops::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetSpreadMethod::<Impl, IMPL_OFFSET>,
            SetSpreadMethod::<Impl, IMPL_OFFSET>,
            GetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            SetColorInterpolationMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGradientStopImpl: Sized {
    fn GetOwner();
    fn GetOffset();
    fn SetOffset();
    fn GetColor();
    fn SetColor();
    fn Clone();
}
impl IXpsOMGradientStopVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStopImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStopVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGradientStopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetOffset::<Impl, IMPL_OFFSET>, SetOffset::<Impl, IMPL_OFFSET>, GetColor::<Impl, IMPL_OFFSET>, SetColor::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientStop as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMGradientStopCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMGradientStopCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMGradientStopCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMGradientStopCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMGradientStopCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMImageBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetImageResource();
    fn SetImageResource();
    fn GetColorProfileResource();
    fn SetColorProfileResource();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMImageBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageBrushVtbl {
        unsafe extern "system" fn GetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMImageBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetViewbox::<Impl, IMPL_OFFSET>,
            SetViewbox::<Impl, IMPL_OFFSET>,
            GetViewport::<Impl, IMPL_OFFSET>,
            SetViewport::<Impl, IMPL_OFFSET>,
            GetTileMode::<Impl, IMPL_OFFSET>,
            SetTileMode::<Impl, IMPL_OFFSET>,
            GetImageResource::<Impl, IMPL_OFFSET>,
            SetImageResource::<Impl, IMPL_OFFSET>,
            GetColorProfileResource::<Impl, IMPL_OFFSET>,
            SetColorProfileResource::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetImageType();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageType<Impl: IXpsOMImageResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>, GetImageType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMImageResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMImageResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMImageResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMImageResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, GetByPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMImageResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMLinearGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetStartPoint();
    fn SetStartPoint();
    fn GetEndPoint();
    fn SetEndPoint();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMLinearGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMLinearGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMLinearGradientBrushVtbl {
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetGradientStops::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetSpreadMethod::<Impl, IMPL_OFFSET>,
            SetSpreadMethod::<Impl, IMPL_OFFSET>,
            GetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            SetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            GetStartPoint::<Impl, IMPL_OFFSET>,
            SetStartPoint::<Impl, IMPL_OFFSET>,
            GetEndPoint::<Impl, IMPL_OFFSET>,
            SetEndPoint::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMLinearGradientBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMMatrixTransformImpl: Sized + IXpsOMShareableImpl {
    fn GetMatrix();
    fn SetMatrix();
    fn Clone();
}
impl IXpsOMMatrixTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMMatrixTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMMatrixTransformVtbl {
        unsafe extern "system" fn GetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMMatrixTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetMatrix::<Impl, IMPL_OFFSET>, SetMatrix::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMMatrixTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMNameCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMNameCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMNameCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMNameCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMNameCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMNameCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMNameCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactoryImpl: Sized {
    fn CreatePackage();
    fn CreatePackageFromFile();
    fn CreatePackageFromStream();
    fn CreateStoryFragmentsResource();
    fn CreateDocumentStructureResource();
    fn CreateSignatureBlockResource();
    fn CreateRemoteDictionaryResource();
    fn CreateRemoteDictionaryResourceFromStream();
    fn CreatePartResources();
    fn CreateDocumentSequence();
    fn CreateDocument();
    fn CreatePageReference();
    fn CreatePage();
    fn CreatePageFromStream();
    fn CreateCanvas();
    fn CreateGlyphs();
    fn CreatePath();
    fn CreateGeometry();
    fn CreateGeometryFigure();
    fn CreateMatrixTransform();
    fn CreateSolidColorBrush();
    fn CreateColorProfileResource();
    fn CreateImageBrush();
    fn CreateVisualBrush();
    fn CreateImageResource();
    fn CreatePrintTicketResource();
    fn CreateFontResource();
    fn CreateGradientStop();
    fn CreateLinearGradientBrush();
    fn CreateRadialGradientBrush();
    fn CreateCoreProperties();
    fn CreateDictionary();
    fn CreatePartUriCollection();
    fn CreatePackageWriterOnFile();
    fn CreatePackageWriterOnStream();
    fn CreatePartUri();
    fn CreateReadOnlyStreamOnFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactoryVtbl {
        unsafe extern "system" fn CreatePackage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageFromFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePartResources<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDocumentSequence<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDocument<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePageReference<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCanvas<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGlyphs<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePath<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometry<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeometryFigure<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateColorProfileResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateImageBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVisualBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateImageResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePrintTicketResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFontResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGradientStop<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCoreProperties<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDictionary<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePartUriCollection<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePartUri<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreatePackage::<Impl, IMPL_OFFSET>,
            CreatePackageFromFile::<Impl, IMPL_OFFSET>,
            CreatePackageFromStream::<Impl, IMPL_OFFSET>,
            CreateStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            CreateDocumentStructureResource::<Impl, IMPL_OFFSET>,
            CreateSignatureBlockResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResourceFromStream::<Impl, IMPL_OFFSET>,
            CreatePartResources::<Impl, IMPL_OFFSET>,
            CreateDocumentSequence::<Impl, IMPL_OFFSET>,
            CreateDocument::<Impl, IMPL_OFFSET>,
            CreatePageReference::<Impl, IMPL_OFFSET>,
            CreatePage::<Impl, IMPL_OFFSET>,
            CreatePageFromStream::<Impl, IMPL_OFFSET>,
            CreateCanvas::<Impl, IMPL_OFFSET>,
            CreateGlyphs::<Impl, IMPL_OFFSET>,
            CreatePath::<Impl, IMPL_OFFSET>,
            CreateGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryFigure::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateColorProfileResource::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateVisualBrush::<Impl, IMPL_OFFSET>,
            CreateImageResource::<Impl, IMPL_OFFSET>,
            CreatePrintTicketResource::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateGradientStop::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCoreProperties::<Impl, IMPL_OFFSET>,
            CreateDictionary::<Impl, IMPL_OFFSET>,
            CreatePartUriCollection::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnFile::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnStream::<Impl, IMPL_OFFSET>,
            CreatePartUri::<Impl, IMPL_OFFSET>,
            CreateReadOnlyStreamOnFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1Impl: Sized + IXpsOMObjectFactoryImpl {
    fn GetDocumentTypeFromFile();
    fn GetDocumentTypeFromStream();
    fn ConvertHDPhotoToJpegXR();
    fn ConvertJpegXRToHDPhoto();
    fn CreatePackageWriterOnFile1();
    fn CreatePackageWriterOnStream1();
    fn CreatePackage1();
    fn CreatePackageFromStream1();
    fn CreatePackageFromFile1();
    fn CreatePage1();
    fn CreatePageFromStream1();
    fn CreateRemoteDictionaryResourceFromStream1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMObjectFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMObjectFactory1Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePackageFromFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreatePackage::<Impl, IMPL_OFFSET>,
            CreatePackageFromFile::<Impl, IMPL_OFFSET>,
            CreatePackageFromStream::<Impl, IMPL_OFFSET>,
            CreateStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            CreateDocumentStructureResource::<Impl, IMPL_OFFSET>,
            CreateSignatureBlockResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResource::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResourceFromStream::<Impl, IMPL_OFFSET>,
            CreatePartResources::<Impl, IMPL_OFFSET>,
            CreateDocumentSequence::<Impl, IMPL_OFFSET>,
            CreateDocument::<Impl, IMPL_OFFSET>,
            CreatePageReference::<Impl, IMPL_OFFSET>,
            CreatePage::<Impl, IMPL_OFFSET>,
            CreatePageFromStream::<Impl, IMPL_OFFSET>,
            CreateCanvas::<Impl, IMPL_OFFSET>,
            CreateGlyphs::<Impl, IMPL_OFFSET>,
            CreatePath::<Impl, IMPL_OFFSET>,
            CreateGeometry::<Impl, IMPL_OFFSET>,
            CreateGeometryFigure::<Impl, IMPL_OFFSET>,
            CreateMatrixTransform::<Impl, IMPL_OFFSET>,
            CreateSolidColorBrush::<Impl, IMPL_OFFSET>,
            CreateColorProfileResource::<Impl, IMPL_OFFSET>,
            CreateImageBrush::<Impl, IMPL_OFFSET>,
            CreateVisualBrush::<Impl, IMPL_OFFSET>,
            CreateImageResource::<Impl, IMPL_OFFSET>,
            CreatePrintTicketResource::<Impl, IMPL_OFFSET>,
            CreateFontResource::<Impl, IMPL_OFFSET>,
            CreateGradientStop::<Impl, IMPL_OFFSET>,
            CreateLinearGradientBrush::<Impl, IMPL_OFFSET>,
            CreateRadialGradientBrush::<Impl, IMPL_OFFSET>,
            CreateCoreProperties::<Impl, IMPL_OFFSET>,
            CreateDictionary::<Impl, IMPL_OFFSET>,
            CreatePartUriCollection::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnFile::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnStream::<Impl, IMPL_OFFSET>,
            CreatePartUri::<Impl, IMPL_OFFSET>,
            CreateReadOnlyStreamOnFile::<Impl, IMPL_OFFSET>,
            GetDocumentTypeFromFile::<Impl, IMPL_OFFSET>,
            GetDocumentTypeFromStream::<Impl, IMPL_OFFSET>,
            ConvertHDPhotoToJpegXR::<Impl, IMPL_OFFSET>,
            ConvertJpegXRToHDPhoto::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnFile1::<Impl, IMPL_OFFSET>,
            CreatePackageWriterOnStream1::<Impl, IMPL_OFFSET>,
            CreatePackage1::<Impl, IMPL_OFFSET>,
            CreatePackageFromStream1::<Impl, IMPL_OFFSET>,
            CreatePackageFromFile1::<Impl, IMPL_OFFSET>,
            CreatePage1::<Impl, IMPL_OFFSET>,
            CreatePageFromStream1::<Impl, IMPL_OFFSET>,
            CreateRemoteDictionaryResourceFromStream1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageImpl: Sized {
    fn GetDocumentSequence();
    fn SetDocumentSequence();
    fn GetCoreProperties();
    fn SetCoreProperties();
    fn GetDiscardControlPartName();
    fn SetDiscardControlPartName();
    fn GetThumbnailResource();
    fn SetThumbnailResource();
    fn WriteToFile();
    fn WriteToStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageVtbl {
        unsafe extern "system" fn GetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteToFile<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteToStream<Impl: IXpsOMPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDocumentSequence::<Impl, IMPL_OFFSET>,
            SetDocumentSequence::<Impl, IMPL_OFFSET>,
            GetCoreProperties::<Impl, IMPL_OFFSET>,
            SetCoreProperties::<Impl, IMPL_OFFSET>,
            GetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            SetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            GetThumbnailResource::<Impl, IMPL_OFFSET>,
            SetThumbnailResource::<Impl, IMPL_OFFSET>,
            WriteToFile::<Impl, IMPL_OFFSET>,
            WriteToStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1Impl: Sized + IXpsOMPackageImpl {
    fn GetDocumentType();
    fn WriteToFile1();
    fn WriteToStream1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackage1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteToFile1<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteToStream1<Impl: IXpsOMPackage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDocumentSequence::<Impl, IMPL_OFFSET>,
            SetDocumentSequence::<Impl, IMPL_OFFSET>,
            GetCoreProperties::<Impl, IMPL_OFFSET>,
            SetCoreProperties::<Impl, IMPL_OFFSET>,
            GetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            SetDiscardControlPartName::<Impl, IMPL_OFFSET>,
            GetThumbnailResource::<Impl, IMPL_OFFSET>,
            SetThumbnailResource::<Impl, IMPL_OFFSET>,
            WriteToFile::<Impl, IMPL_OFFSET>,
            WriteToStream::<Impl, IMPL_OFFSET>,
            GetDocumentType::<Impl, IMPL_OFFSET>,
            WriteToFile1::<Impl, IMPL_OFFSET>,
            WriteToStream1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackage1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMPackageTargetImpl: Sized {
    fn CreateXpsOMPackageWriter();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageTargetVtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Impl: IXpsOMPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateXpsOMPackageWriter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsOMPackageWriterImpl: Sized {
    fn StartNewDocument();
    fn AddPage();
    fn AddResource();
    fn Close();
    fn IsClosed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsOMPackageWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriterVtbl {
        unsafe extern "system" fn StartNewDocument<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPage<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddResource<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClosed<Impl: IXpsOMPackageWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StartNewDocument::<Impl, IMPL_OFFSET>, AddPage::<Impl, IMPL_OFFSET>, AddResource::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, IsClosed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3DImpl: Sized + IXpsOMPackageWriterImpl {
    fn AddModelTexture();
    fn SetModelPrintTicket();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPackageWriter3DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPackageWriter3DVtbl {
        unsafe extern "system" fn AddModelTexture<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModelPrintTicket<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StartNewDocument::<Impl, IMPL_OFFSET>, AddPage::<Impl, IMPL_OFFSET>, AddResource::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, IsClosed::<Impl, IMPL_OFFSET>, AddModelTexture::<Impl, IMPL_OFFSET>, SetModelPrintTicket::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPageImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetVisuals();
    fn GetPageDimensions();
    fn SetPageDimensions();
    fn GetContentBox();
    fn SetContentBox();
    fn GetBleedBox();
    fn SetBleedBox();
    fn GetLanguage();
    fn SetLanguage();
    fn GetName();
    fn SetName();
    fn GetIsHyperlinkTarget();
    fn SetIsHyperlinkTarget();
    fn GetDictionary();
    fn GetDictionaryLocal();
    fn SetDictionaryLocal();
    fn GetDictionaryResource();
    fn SetDictionaryResource();
    fn Write();
    fn GenerateUnusedLookupKey();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPartName::<Impl, IMPL_OFFSET>,
            SetPartName::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetVisuals::<Impl, IMPL_OFFSET>,
            GetPageDimensions::<Impl, IMPL_OFFSET>,
            SetPageDimensions::<Impl, IMPL_OFFSET>,
            GetContentBox::<Impl, IMPL_OFFSET>,
            SetContentBox::<Impl, IMPL_OFFSET>,
            GetBleedBox::<Impl, IMPL_OFFSET>,
            SetBleedBox::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetDictionary::<Impl, IMPL_OFFSET>,
            GetDictionaryLocal::<Impl, IMPL_OFFSET>,
            SetDictionaryLocal::<Impl, IMPL_OFFSET>,
            GetDictionaryResource::<Impl, IMPL_OFFSET>,
            SetDictionaryResource::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            GenerateUnusedLookupKey::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1Impl: Sized + IXpsOMPageImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPage1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMPage1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPartName::<Impl, IMPL_OFFSET>,
            SetPartName::<Impl, IMPL_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetVisuals::<Impl, IMPL_OFFSET>,
            GetPageDimensions::<Impl, IMPL_OFFSET>,
            SetPageDimensions::<Impl, IMPL_OFFSET>,
            GetContentBox::<Impl, IMPL_OFFSET>,
            SetContentBox::<Impl, IMPL_OFFSET>,
            GetBleedBox::<Impl, IMPL_OFFSET>,
            SetBleedBox::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetDictionary::<Impl, IMPL_OFFSET>,
            GetDictionaryLocal::<Impl, IMPL_OFFSET>,
            SetDictionaryLocal::<Impl, IMPL_OFFSET>,
            GetDictionaryResource::<Impl, IMPL_OFFSET>,
            SetDictionaryResource::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            GenerateUnusedLookupKey::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            GetDocumentType::<Impl, IMPL_OFFSET>,
            Write1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPage1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMPageReferenceImpl: Sized {
    fn GetOwner();
    fn GetPage();
    fn SetPage();
    fn DiscardPage();
    fn IsPageLoaded();
    fn GetAdvisoryPageDimensions();
    fn SetAdvisoryPageDimensions();
    fn GetStoryFragmentsResource();
    fn SetStoryFragmentsResource();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
    fn GetThumbnailResource();
    fn SetThumbnailResource();
    fn CollectLinkTargets();
    fn CollectPartResources();
    fn HasRestrictedFonts();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMPageReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReferenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscardPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPageLoaded<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectLinkTargets<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CollectPartResources<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasRestrictedFonts<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetPage::<Impl, IMPL_OFFSET>,
            SetPage::<Impl, IMPL_OFFSET>,
            DiscardPage::<Impl, IMPL_OFFSET>,
            IsPageLoaded::<Impl, IMPL_OFFSET>,
            GetAdvisoryPageDimensions::<Impl, IMPL_OFFSET>,
            SetAdvisoryPageDimensions::<Impl, IMPL_OFFSET>,
            GetStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            SetStoryFragmentsResource::<Impl, IMPL_OFFSET>,
            GetPrintTicketResource::<Impl, IMPL_OFFSET>,
            SetPrintTicketResource::<Impl, IMPL_OFFSET>,
            GetThumbnailResource::<Impl, IMPL_OFFSET>,
            SetThumbnailResource::<Impl, IMPL_OFFSET>,
            CollectLinkTargets::<Impl, IMPL_OFFSET>,
            CollectPartResources::<Impl, IMPL_OFFSET>,
            HasRestrictedFonts::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPageReference as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMPageReferenceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMPageReferenceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPageReferenceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPageReferenceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPageReferenceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMPartImpl: Sized {
    fn GetPartName();
    fn SetPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMPartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartVtbl {
        unsafe extern "system" fn GetPartName<Impl: IXpsOMPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPartName<Impl: IXpsOMPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPart as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMPartResourcesImpl: Sized {
    fn GetFontResources();
    fn GetImageResources();
    fn GetColorProfileResources();
    fn GetRemoteDictionaryResources();
}
impl IXpsOMPartResourcesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartResourcesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartResourcesVtbl {
        unsafe extern "system" fn GetFontResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorProfileResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFontResources::<Impl, IMPL_OFFSET>, GetImageResources::<Impl, IMPL_OFFSET>, GetColorProfileResources::<Impl, IMPL_OFFSET>, GetRemoteDictionaryResources::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPartResources as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMPartUriCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMPartUriCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPartUriCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPartUriCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPartUriCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMPathImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetGeometry();
    fn GetGeometryLocal();
    fn SetGeometryLocal();
    fn GetGeometryLookup();
    fn SetGeometryLookup();
    fn GetAccessibilityShortDescription();
    fn SetAccessibilityShortDescription();
    fn GetAccessibilityLongDescription();
    fn SetAccessibilityLongDescription();
    fn GetSnapsToPixels();
    fn SetSnapsToPixels();
    fn GetStrokeBrush();
    fn GetStrokeBrushLocal();
    fn SetStrokeBrushLocal();
    fn GetStrokeBrushLookup();
    fn SetStrokeBrushLookup();
    fn GetStrokeDashes();
    fn GetStrokeDashCap();
    fn SetStrokeDashCap();
    fn GetStrokeDashOffset();
    fn SetStrokeDashOffset();
    fn GetStrokeStartLineCap();
    fn SetStrokeStartLineCap();
    fn GetStrokeEndLineCap();
    fn SetStrokeEndLineCap();
    fn GetStrokeLineJoin();
    fn SetStrokeLineJoin();
    fn GetStrokeMiterLimit();
    fn SetStrokeMiterLimit();
    fn GetStrokeThickness();
    fn SetStrokeThickness();
    fn GetFillBrush();
    fn GetFillBrushLocal();
    fn SetFillBrushLocal();
    fn GetFillBrushLookup();
    fn SetFillBrushLookup();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPathVtbl {
        unsafe extern "system" fn GetGeometry<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeBrush<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeDashes<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetClipGeometry::<Impl, IMPL_OFFSET>,
            GetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            SetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            GetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            SetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrush::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            SetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
            GetGeometry::<Impl, IMPL_OFFSET>,
            GetGeometryLocal::<Impl, IMPL_OFFSET>,
            SetGeometryLocal::<Impl, IMPL_OFFSET>,
            GetGeometryLookup::<Impl, IMPL_OFFSET>,
            SetGeometryLookup::<Impl, IMPL_OFFSET>,
            GetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityShortDescription::<Impl, IMPL_OFFSET>,
            GetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            SetAccessibilityLongDescription::<Impl, IMPL_OFFSET>,
            GetSnapsToPixels::<Impl, IMPL_OFFSET>,
            SetSnapsToPixels::<Impl, IMPL_OFFSET>,
            GetStrokeBrush::<Impl, IMPL_OFFSET>,
            GetStrokeBrushLocal::<Impl, IMPL_OFFSET>,
            SetStrokeBrushLocal::<Impl, IMPL_OFFSET>,
            GetStrokeBrushLookup::<Impl, IMPL_OFFSET>,
            SetStrokeBrushLookup::<Impl, IMPL_OFFSET>,
            GetStrokeDashes::<Impl, IMPL_OFFSET>,
            GetStrokeDashCap::<Impl, IMPL_OFFSET>,
            SetStrokeDashCap::<Impl, IMPL_OFFSET>,
            GetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            SetStrokeDashOffset::<Impl, IMPL_OFFSET>,
            GetStrokeStartLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeStartLineCap::<Impl, IMPL_OFFSET>,
            GetStrokeEndLineCap::<Impl, IMPL_OFFSET>,
            SetStrokeEndLineCap::<Impl, IMPL_OFFSET>,
            GetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            SetStrokeLineJoin::<Impl, IMPL_OFFSET>,
            GetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            SetStrokeMiterLimit::<Impl, IMPL_OFFSET>,
            GetStrokeThickness::<Impl, IMPL_OFFSET>,
            SetStrokeThickness::<Impl, IMPL_OFFSET>,
            GetFillBrush::<Impl, IMPL_OFFSET>,
            GetFillBrushLocal::<Impl, IMPL_OFFSET>,
            SetFillBrushLocal::<Impl, IMPL_OFFSET>,
            GetFillBrushLookup::<Impl, IMPL_OFFSET>,
            SetFillBrushLookup::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMPrintTicketResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMPrintTicketResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMRadialGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetCenter();
    fn SetCenter();
    fn GetRadiiSizes();
    fn SetRadiiSizes();
    fn GetGradientOrigin();
    fn SetGradientOrigin();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMRadialGradientBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRadialGradientBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRadialGradientBrushVtbl {
        unsafe extern "system" fn GetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetGradientStops::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetSpreadMethod::<Impl, IMPL_OFFSET>,
            SetSpreadMethod::<Impl, IMPL_OFFSET>,
            GetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            SetColorInterpolationMode::<Impl, IMPL_OFFSET>,
            GetCenter::<Impl, IMPL_OFFSET>,
            SetCenter::<Impl, IMPL_OFFSET>,
            GetRadiiSizes::<Impl, IMPL_OFFSET>,
            SetRadiiSizes::<Impl, IMPL_OFFSET>,
            GetGradientOrigin::<Impl, IMPL_OFFSET>,
            SetGradientOrigin::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRadialGradientBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMRemoteDictionaryResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDictionary();
    fn SetDictionary();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMRemoteDictionaryResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceVtbl {
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetDictionary::<Impl, IMPL_OFFSET>, SetDictionary::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1Impl: Sized + IXpsOMRemoteDictionaryResourceImpl + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResource1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResource1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetDictionary::<Impl, IMPL_OFFSET>, SetDictionary::<Impl, IMPL_OFFSET>, GetDocumentType::<Impl, IMPL_OFFSET>, Write1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMRemoteDictionaryResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMRemoteDictionaryResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMRemoteDictionaryResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, GetByPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMResourceImpl: Sized + IXpsOMPartImpl {}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMResourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMResource as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMShareableImpl: Sized {
    fn GetOwner();
    fn GetType();
}
impl IXpsOMShareableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMShareableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMShareableVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMShareableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IXpsOMShareableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMShareable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMSignatureBlockResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMSignatureBlockResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSignatureBlockResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSignatureBlockResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, GetByPartName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResourceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMSolidColorBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetColor();
    fn SetColor();
    fn Clone();
}
impl IXpsOMSolidColorBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMSolidColorBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMSolidColorBrushVtbl {
        unsafe extern "system" fn GetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, GetOpacity::<Impl, IMPL_OFFSET>, SetOpacity::<Impl, IMPL_OFFSET>, GetColor::<Impl, IMPL_OFFSET>, SetColor::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMSolidColorBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMStoryFragmentsResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMStoryFragmentsResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, SetPartName::<Impl, IMPL_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, SetContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMStoryFragmentsResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub trait IXpsOMThumbnailGeneratorImpl: Sized {
    fn GenerateThumbnail();
}
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
impl IXpsOMThumbnailGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMThumbnailGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMThumbnailGeneratorVtbl {
        unsafe extern "system" fn GenerateThumbnail<Impl: IXpsOMThumbnailGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GenerateThumbnail::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMThumbnailGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMTileBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetViewbox();
    fn SetViewbox();
    fn GetViewport();
    fn SetViewport();
    fn GetTileMode();
    fn SetTileMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMTileBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMTileBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMTileBrushVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetViewbox::<Impl, IMPL_OFFSET>,
            SetViewbox::<Impl, IMPL_OFFSET>,
            GetViewport::<Impl, IMPL_OFFSET>,
            SetViewport::<Impl, IMPL_OFFSET>,
            GetTileMode::<Impl, IMPL_OFFSET>,
            SetTileMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMTileBrush as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMVisualImpl: Sized + IXpsOMShareableImpl {
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetClipGeometry();
    fn GetClipGeometryLocal();
    fn SetClipGeometryLocal();
    fn GetClipGeometryLookup();
    fn SetClipGeometryLookup();
    fn GetOpacity();
    fn SetOpacity();
    fn GetOpacityMaskBrush();
    fn GetOpacityMaskBrushLocal();
    fn SetOpacityMaskBrushLocal();
    fn GetOpacityMaskBrushLookup();
    fn SetOpacityMaskBrushLookup();
    fn GetName();
    fn SetName();
    fn GetIsHyperlinkTarget();
    fn SetIsHyperlinkTarget();
    fn GetHyperlinkNavigateUri();
    fn SetHyperlinkNavigateUri();
    fn GetLanguage();
    fn SetLanguage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXpsOMVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipGeometry<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetClipGeometry::<Impl, IMPL_OFFSET>,
            GetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            SetClipGeometryLocal::<Impl, IMPL_OFFSET>,
            GetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            SetClipGeometryLookup::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrush::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLocal::<Impl, IMPL_OFFSET>,
            GetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            SetOpacityMaskBrushLookup::<Impl, IMPL_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            GetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            SetIsHyperlinkTarget::<Impl, IMPL_OFFSET>,
            GetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            SetHyperlinkNavigateUri::<Impl, IMPL_OFFSET>,
            GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMVisualBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetVisual();
    fn GetVisualLocal();
    fn SetVisualLocal();
    fn GetVisualLookup();
    fn SetVisualLookup();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IXpsOMVisualBrushVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualBrushImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualBrushVtbl {
        unsafe extern "system" fn GetVisual<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMVisualBrushImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwner::<Impl, IMPL_OFFSET>,
            GetType::<Impl, IMPL_OFFSET>,
            GetOpacity::<Impl, IMPL_OFFSET>,
            SetOpacity::<Impl, IMPL_OFFSET>,
            GetTransform::<Impl, IMPL_OFFSET>,
            GetTransformLocal::<Impl, IMPL_OFFSET>,
            SetTransformLocal::<Impl, IMPL_OFFSET>,
            GetTransformLookup::<Impl, IMPL_OFFSET>,
            SetTransformLookup::<Impl, IMPL_OFFSET>,
            GetViewbox::<Impl, IMPL_OFFSET>,
            SetViewbox::<Impl, IMPL_OFFSET>,
            GetViewport::<Impl, IMPL_OFFSET>,
            SetViewport::<Impl, IMPL_OFFSET>,
            GetTileMode::<Impl, IMPL_OFFSET>,
            SetTileMode::<Impl, IMPL_OFFSET>,
            GetVisual::<Impl, IMPL_OFFSET>,
            GetVisualLocal::<Impl, IMPL_OFFSET>,
            SetVisualLocal::<Impl, IMPL_OFFSET>,
            GetVisualLookup::<Impl, IMPL_OFFSET>,
            SetVisualLookup::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisualBrush as ::windows::core::Interface>::IID
    }
}
pub trait IXpsOMVisualCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl IXpsOMVisualCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsOMVisualCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsOMVisualCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IXpsOMVisualCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, SetAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsOMVisualCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsSignatureImpl: Sized {
    fn GetSignatureId();
    fn GetSignatureValue();
    fn GetCertificateEnumerator();
    fn GetSigningTime();
    fn GetSigningTimeFormat();
    fn GetSignaturePartName();
    fn Verify();
    fn GetPolicy();
    fn GetCustomObjectEnumerator();
    fn GetCustomReferenceEnumerator();
    fn GetSignatureXml();
    fn SetSignatureXml();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningTime<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Verify<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignatureValue::<Impl, IMPL_OFFSET>,
            GetCertificateEnumerator::<Impl, IMPL_OFFSET>,
            GetSigningTime::<Impl, IMPL_OFFSET>,
            GetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            GetSignaturePartName::<Impl, IMPL_OFFSET>,
            Verify::<Impl, IMPL_OFFSET>,
            GetPolicy::<Impl, IMPL_OFFSET>,
            GetCustomObjectEnumerator::<Impl, IMPL_OFFSET>,
            GetCustomReferenceEnumerator::<Impl, IMPL_OFFSET>,
            GetSignatureXml::<Impl, IMPL_OFFSET>,
            SetSignatureXml::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsSignatureBlockImpl: Sized {
    fn GetRequests();
    fn GetPartName();
    fn GetDocumentIndex();
    fn GetDocumentName();
    fn CreateRequest();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsSignatureBlockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlockVtbl {
        unsafe extern "system" fn GetRequests<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPartName<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentIndex<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentName<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRequest<Impl: IXpsSignatureBlockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRequests::<Impl, IMPL_OFFSET>, GetPartName::<Impl, IMPL_OFFSET>, GetDocumentIndex::<Impl, IMPL_OFFSET>, GetDocumentName::<Impl, IMPL_OFFSET>, CreateRequest::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureBlock as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureBlockCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl IXpsSignatureBlockCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureBlockCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureBlockCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureBlockCollection as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl IXpsSignatureCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManagerImpl: Sized {
    fn LoadPackageFile();
    fn LoadPackageStream();
    fn Sign();
    fn GetSignatureOriginPartName();
    fn SetSignatureOriginPartName();
    fn GetSignatures();
    fn AddSignatureBlock();
    fn GetSignatureBlocks();
    fn CreateSigningOptions();
    fn SavePackageToFile();
    fn SavePackageToStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureManagerVtbl {
        unsafe extern "system" fn LoadPackageFile<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadPackageStream<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Sign<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatures<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSignatureBlock<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureBlocks<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SavePackageToFile<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SavePackageToStream<Impl: IXpsSignatureManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            LoadPackageFile::<Impl, IMPL_OFFSET>,
            LoadPackageStream::<Impl, IMPL_OFFSET>,
            Sign::<Impl, IMPL_OFFSET>,
            GetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            SetSignatureOriginPartName::<Impl, IMPL_OFFSET>,
            GetSignatures::<Impl, IMPL_OFFSET>,
            AddSignatureBlock::<Impl, IMPL_OFFSET>,
            GetSignatureBlocks::<Impl, IMPL_OFFSET>,
            CreateSigningOptions::<Impl, IMPL_OFFSET>,
            SavePackageToFile::<Impl, IMPL_OFFSET>,
            SavePackageToStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsSignatureRequestImpl: Sized {
    fn GetIntent();
    fn SetIntent();
    fn GetRequestedSigner();
    fn SetRequestedSigner();
    fn GetRequestSignByDate();
    fn SetRequestSignByDate();
    fn GetSigningLocale();
    fn SetSigningLocale();
    fn GetSpotLocation();
    fn SetSpotLocation();
    fn GetRequestId();
    fn GetSignature();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsSignatureRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequestVtbl {
        unsafe extern "system" fn GetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, place: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRequestId<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignature<Impl: IXpsSignatureRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetIntent::<Impl, IMPL_OFFSET>,
            SetIntent::<Impl, IMPL_OFFSET>,
            GetRequestedSigner::<Impl, IMPL_OFFSET>,
            SetRequestedSigner::<Impl, IMPL_OFFSET>,
            GetRequestSignByDate::<Impl, IMPL_OFFSET>,
            SetRequestSignByDate::<Impl, IMPL_OFFSET>,
            GetSigningLocale::<Impl, IMPL_OFFSET>,
            SetSigningLocale::<Impl, IMPL_OFFSET>,
            GetSpotLocation::<Impl, IMPL_OFFSET>,
            SetSpotLocation::<Impl, IMPL_OFFSET>,
            GetRequestId::<Impl, IMPL_OFFSET>,
            GetSignature::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureRequest as ::windows::core::Interface>::IID
    }
}
pub trait IXpsSignatureRequestCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl IXpsSignatureRequestCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSignatureRequestCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSignatureRequestCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSignatureRequestCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
pub trait IXpsSigningOptionsImpl: Sized {
    fn GetSignatureId();
    fn SetSignatureId();
    fn GetSignatureMethod();
    fn SetSignatureMethod();
    fn GetDigestMethod();
    fn SetDigestMethod();
    fn GetSignaturePartName();
    fn SetSignaturePartName();
    fn GetPolicy();
    fn SetPolicy();
    fn GetSigningTimeFormat();
    fn SetSigningTimeFormat();
    fn GetCustomObjects();
    fn GetCustomReferences();
    fn GetCertificateSet();
    fn GetFlags();
    fn SetFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc"))]
impl IXpsSigningOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsSigningOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsSigningOptionsVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomObjects<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustomReferences<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSignatureId::<Impl, IMPL_OFFSET>,
            SetSignatureId::<Impl, IMPL_OFFSET>,
            GetSignatureMethod::<Impl, IMPL_OFFSET>,
            SetSignatureMethod::<Impl, IMPL_OFFSET>,
            GetDigestMethod::<Impl, IMPL_OFFSET>,
            SetDigestMethod::<Impl, IMPL_OFFSET>,
            GetSignaturePartName::<Impl, IMPL_OFFSET>,
            SetSignaturePartName::<Impl, IMPL_OFFSET>,
            GetPolicy::<Impl, IMPL_OFFSET>,
            SetPolicy::<Impl, IMPL_OFFSET>,
            GetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            SetSigningTimeFormat::<Impl, IMPL_OFFSET>,
            GetCustomObjects::<Impl, IMPL_OFFSET>,
            GetCustomReferences::<Impl, IMPL_OFFSET>,
            GetCertificateSet::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsSigningOptions as ::windows::core::Interface>::IID
    }
}
