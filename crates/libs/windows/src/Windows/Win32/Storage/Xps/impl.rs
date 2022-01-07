pub trait IXpsDocumentPackageTargetImpl: Sized {
    fn GetXpsOMPackageWriter();
    fn GetXpsOMFactory();
    fn GetXpsType();
}
impl ::windows::core::RuntimeName for IXpsDocumentPackageTarget {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsDocumentPackageTarget";
}
impl IXpsDocumentPackageTargetVtbl {
    pub const fn new<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsDocumentPackageTargetVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter(&*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&packagewriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXpsOMFactory(::core::mem::transmute_copy(&xpsfactory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsType<Impl: IXpsDocumentPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXpsType(::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsDocumentPackageTarget>, base.5, GetXpsOMPackageWriter::<Impl, OFFSET>, GetXpsOMFactory::<Impl, OFFSET>, GetXpsType::<Impl, OFFSET>)
    }
}
pub trait IXpsDocumentPackageTarget3DImpl: Sized {
    fn GetXpsOMPackageWriter3D();
    fn GetXpsOMFactory();
}
impl ::windows::core::RuntimeName for IXpsDocumentPackageTarget3D {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsDocumentPackageTarget3D";
}
impl IXpsDocumentPackageTarget3DVtbl {
    pub const fn new<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsDocumentPackageTarget3DVtbl {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, modelpartname: ::windows::core::RawPtr, modeldata: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXpsOMPackageWriter3D(
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&modelpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&modeldata as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Impl: IXpsDocumentPackageTarget3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetXpsOMFactory(::core::mem::transmute_copy(&xpsfactory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsDocumentPackageTarget3D>, base.5, GetXpsOMPackageWriter3D::<Impl, OFFSET>, GetXpsOMFactory::<Impl, OFFSET>)
    }
}
pub trait IXpsOMBrushImpl: Sized + IXpsOMShareableImpl {
    fn GetOpacity();
    fn SetOpacity();
}
impl ::windows::core::RuntimeName for IXpsOMBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMBrush";
}
impl IXpsOMBrushVtbl {
    pub const fn new<Impl: IXpsOMBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMBrushVtbl {
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacity(::core::mem::transmute_copy(&opacity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOpacity(opacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMBrush>, base.5, GetOpacity::<Impl, OFFSET>, SetOpacity::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMCanvas {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMCanvas";
}
impl IXpsOMCanvasVtbl {
    pub const fn new<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMCanvasVtbl {
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisuals(::core::mem::transmute_copy(&visuals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUseAliasedEdgeMode(::core::mem::transmute_copy(&usealiasededgemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUseAliasedEdgeMode(&*(&usealiasededgemode as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccessibilityShortDescription(&*(&shortdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccessibilityLongDescription(&*(&longdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionary(::core::mem::transmute_copy(&resourcedictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal(::core::mem::transmute_copy(&resourcedictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDictionaryLocal(&*(&resourcedictionary as *const <IXpsOMDictionary as ::windows::core::Abi>::Abi as *const <IXpsOMDictionary as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource(::core::mem::transmute_copy(&remotedictionaryresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDictionaryResource(&*(&remotedictionaryresource as *const <IXpsOMRemoteDictionaryResource as ::windows::core::Abi>::Abi as *const <IXpsOMRemoteDictionaryResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCanvasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&canvas)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMCanvas>,
            base.5,
            GetVisuals::<Impl, OFFSET>,
            GetUseAliasedEdgeMode::<Impl, OFFSET>,
            SetUseAliasedEdgeMode::<Impl, OFFSET>,
            GetAccessibilityShortDescription::<Impl, OFFSET>,
            SetAccessibilityShortDescription::<Impl, OFFSET>,
            GetAccessibilityLongDescription::<Impl, OFFSET>,
            SetAccessibilityLongDescription::<Impl, OFFSET>,
            GetDictionary::<Impl, OFFSET>,
            GetDictionaryLocal::<Impl, OFFSET>,
            SetDictionaryLocal::<Impl, OFFSET>,
            GetDictionaryResource::<Impl, OFFSET>,
            SetDictionaryResource::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsOMColorProfileResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
impl ::windows::core::RuntimeName for IXpsOMColorProfileResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMColorProfileResource";
}
impl IXpsOMColorProfileResourceVtbl {
    pub const fn new<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMColorProfileResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMColorProfileResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMColorProfileResource>, base.5, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>)
    }
}
pub trait IXpsOMColorProfileResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
impl ::windows::core::RuntimeName for IXpsOMColorProfileResourceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMColorProfileResourceCollection";
}
impl IXpsOMColorProfileResourceCollectionVtbl {
    pub const fn new<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMColorProfileResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&object as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&object as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&object as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMColorProfileResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByPartName(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMColorProfileResourceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, GetByPartName::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMCoreProperties {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMCoreProperties";
}
impl IXpsOMCorePropertiesVtbl {
    pub const fn new<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMCorePropertiesVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCategory(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCategory(&*(&category as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentstatus: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentStatus(::core::mem::transmute_copy(&contentstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentStatus<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContentStatus(&*(&contentstatus as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentType(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContentType(&*(&contenttype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCreated(::core::mem::transmute_copy(&created)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreated<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCreated(&*(&created as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCreator(::core::mem::transmute_copy(&creator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCreator(&*(&creator as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&description as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdentifier(::core::mem::transmute_copy(&identifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdentifier<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIdentifier(&*(&identifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keywords: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKeywords(::core::mem::transmute_copy(&keywords)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keywords: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetKeywords(&*(&keywords as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguage(::core::mem::transmute_copy(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLanguage(&*(&language as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastModifiedBy(::core::mem::transmute_copy(&lastmodifiedby)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodifiedby: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLastModifiedBy(&*(&lastmodifiedby as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastPrinted(::core::mem::transmute_copy(&lastprinted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastPrinted<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLastPrinted(&*(&lastprinted as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetModified(::core::mem::transmute_copy(&modified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModified<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetModified(&*(&modified as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, revision: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRevision(::core::mem::transmute_copy(&revision)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, revision: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRevision(&*(&revision as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subject: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubject(::core::mem::transmute_copy(&subject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subject: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSubject(&*(&subject as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTitle(::core::mem::transmute_copy(&title)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTitle(&*(&title as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVersion(&*(&version as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMCorePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&coreproperties)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMCoreProperties>,
            base.5,
            GetOwner::<Impl, OFFSET>,
            GetCategory::<Impl, OFFSET>,
            SetCategory::<Impl, OFFSET>,
            GetContentStatus::<Impl, OFFSET>,
            SetContentStatus::<Impl, OFFSET>,
            GetContentType::<Impl, OFFSET>,
            SetContentType::<Impl, OFFSET>,
            GetCreated::<Impl, OFFSET>,
            SetCreated::<Impl, OFFSET>,
            GetCreator::<Impl, OFFSET>,
            SetCreator::<Impl, OFFSET>,
            GetDescription::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            GetIdentifier::<Impl, OFFSET>,
            SetIdentifier::<Impl, OFFSET>,
            GetKeywords::<Impl, OFFSET>,
            SetKeywords::<Impl, OFFSET>,
            GetLanguage::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
            GetLastModifiedBy::<Impl, OFFSET>,
            SetLastModifiedBy::<Impl, OFFSET>,
            GetLastPrinted::<Impl, OFFSET>,
            SetLastPrinted::<Impl, OFFSET>,
            GetModified::<Impl, OFFSET>,
            SetModified::<Impl, OFFSET>,
            GetRevision::<Impl, OFFSET>,
            SetRevision::<Impl, OFFSET>,
            GetSubject::<Impl, OFFSET>,
            SetSubject::<Impl, OFFSET>,
            GetTitle::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            SetVersion::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IXpsOMDashCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDashCollection";
}
impl IXpsOMDashCollectionVtbl {
    pub const fn new<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDashCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&dash)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&dash as *const <XPS_DASH as ::windows::core::Abi>::Abi as *const <XPS_DASH as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&dash as *const <XPS_DASH as ::windows::core::Abi>::Abi as *const <XPS_DASH as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDashCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&dash as *const <XPS_DASH as ::windows::core::Abi>::Abi as *const <XPS_DASH as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDashCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMDictionary {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDictionary";
}
impl IXpsOMDictionaryVtbl {
    pub const fn new<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDictionaryVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut super::super::Foundation::PWSTR, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByKey<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, beforeentry: ::windows::core::RawPtr, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByKey(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&beforeentry as *const <IXpsOMShareable as ::windows::core::Abi>::Abi as *const <IXpsOMShareable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entry: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIndex(&*(&entry as *const <IXpsOMShareable as ::windows::core::Abi>::Abi as *const <IXpsOMShareable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&entry as *const <IXpsOMShareable as ::windows::core::Abi>::Abi as *const <IXpsOMShareable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&entry as *const <IXpsOMShareable as ::windows::core::Abi>::Abi as *const <IXpsOMShareable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: super::super::Foundation::PWSTR, entry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&entry as *const <IXpsOMShareable as ::windows::core::Abi>::Abi as *const <IXpsOMShareable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&dictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDictionary>, base.5, GetOwner::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetByKey::<Impl, OFFSET>, GetIndex::<Impl, OFFSET>, Append::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMDocument {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDocument";
}
impl IXpsOMDocumentVtbl {
    pub const fn new<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDocumentVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&documentsequence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageReferences<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereferences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPageReferences(::core::mem::transmute_copy(&pagereferences)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource(::core::mem::transmute_copy(&printticketresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrintTicketResource(&*(&printticketresource as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentStructureResource(::core::mem::transmute_copy(&documentstructureresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentstructureresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDocumentStructureResource(&*(&documentstructureresource as *const <IXpsOMDocumentStructureResource as ::windows::core::Abi>::Abi as *const <IXpsOMDocumentStructureResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlockResources<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlockResources(::core::mem::transmute_copy(&signatureblockresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDocument>, base.5, GetOwner::<Impl, OFFSET>, GetPageReferences::<Impl, OFFSET>, GetPrintTicketResource::<Impl, OFFSET>, SetPrintTicketResource::<Impl, OFFSET>, GetDocumentStructureResource::<Impl, OFFSET>, SetDocumentStructureResource::<Impl, OFFSET>, GetSignatureBlockResources::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IXpsOMDocumentCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDocumentCollection";
}
impl IXpsOMDocumentCollectionVtbl {
    pub const fn new<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDocumentCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&document as *const <IXpsOMDocument as ::windows::core::Abi>::Abi as *const <IXpsOMDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&document as *const <IXpsOMDocument as ::windows::core::Abi>::Abi as *const <IXpsOMDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMDocumentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&document as *const <IXpsOMDocument as ::windows::core::Abi>::Abi as *const <IXpsOMDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDocumentCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
pub trait IXpsOMDocumentSequenceImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetDocuments();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
}
impl ::windows::core::RuntimeName for IXpsOMDocumentSequence {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDocumentSequence";
}
impl IXpsOMDocumentSequenceVtbl {
    pub const fn new<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDocumentSequenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocuments(::core::mem::transmute_copy(&documents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource(::core::mem::transmute_copy(&printticketresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMDocumentSequenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrintTicketResource(&*(&printticketresource as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDocumentSequence>, base.5, GetOwner::<Impl, OFFSET>, GetDocuments::<Impl, OFFSET>, GetPrintTicketResource::<Impl, OFFSET>, SetPrintTicketResource::<Impl, OFFSET>)
    }
}
pub trait IXpsOMDocumentStructureResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
impl ::windows::core::RuntimeName for IXpsOMDocumentStructureResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMDocumentStructureResource";
}
impl IXpsOMDocumentStructureResourceVtbl {
    pub const fn new<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMDocumentStructureResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMDocumentStructureResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMDocumentStructureResource>, base.5, GetOwner::<Impl, OFFSET>, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>)
    }
}
pub trait IXpsOMFontResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetEmbeddingOption();
}
impl ::windows::core::RuntimeName for IXpsOMFontResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMFontResource";
}
impl IXpsOMFontResourceVtbl {
    pub const fn new<Impl: IXpsOMFontResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMFontResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMFontResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&readerstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMFontResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, embeddingoption: XPS_FONT_EMBEDDING, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), embeddingoption, &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddingOption<Impl: IXpsOMFontResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEmbeddingOption(::core::mem::transmute_copy(&embeddingoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMFontResource>, base.5, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>, GetEmbeddingOption::<Impl, OFFSET>)
    }
}
pub trait IXpsOMFontResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn SetAt();
    fn InsertAt();
    fn Append();
    fn RemoveAt();
    fn GetByPartName();
}
impl ::windows::core::RuntimeName for IXpsOMFontResourceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMFontResourceCollection";
}
impl IXpsOMFontResourceCollectionVtbl {
    pub const fn new<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMFontResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&value as *const <IXpsOMFontResource as ::windows::core::Abi>::Abi as *const <IXpsOMFontResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&value as *const <IXpsOMFontResource as ::windows::core::Abi>::Abi as *const <IXpsOMFontResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&value as *const <IXpsOMFontResource as ::windows::core::Abi>::Abi as *const <IXpsOMFontResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMFontResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByPartName(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMFontResourceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, GetByPartName::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMGeometry {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGeometry";
}
impl IXpsOMGeometryVtbl {
    pub const fn new<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGeometryVtbl {
        unsafe extern "system" fn GetFigures<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, figures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFigures(::core::mem::transmute_copy(&figures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillRule(::core::mem::transmute_copy(&fillrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillRule<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillRule(fillrule) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLocal(&*(&transform as *const <IXpsOMMatrixTransform as ::windows::core::Abi>::Abi as *const <IXpsOMMatrixTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup(::core::mem::transmute_copy(&lookup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLookup(&*(&lookup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&geometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMGeometry>, base.5, GetFigures::<Impl, OFFSET>, GetFillRule::<Impl, OFFSET>, SetFillRule::<Impl, OFFSET>, GetTransform::<Impl, OFFSET>, GetTransformLocal::<Impl, OFFSET>, SetTransformLocal::<Impl, OFFSET>, GetTransformLookup::<Impl, OFFSET>, SetTransformLookup::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMGeometryFigure {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGeometryFigure";
}
impl IXpsOMGeometryFigureVtbl {
    pub const fn new<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGeometryFigureVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentData<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentData(datacount, segmentdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentTypes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentTypes(segmentcount, segmenttypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokes<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentStrokes(segmentcount, &*(&segmentstrokes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegments<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSegments(segmentcount, segmentdatacount, segmenttypes, segmentdata, &*(&segmentstrokes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStartPoint(::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStartPoint(&*(&startpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsClosed(::core::mem::transmute_copy(&isclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClosed<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsClosed(&*(&isclosed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsFilled(::core::mem::transmute_copy(&isfilled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFilled<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsFilled(&*(&isfilled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentCount(::core::mem::transmute_copy(&segmentcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentDataCount(::core::mem::transmute_copy(&segmentdatacount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSegmentStrokePattern(::core::mem::transmute_copy(&segmentstrokepattern)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGeometryFigureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&geometryfigure)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMGeometryFigure>,
            base.5,
            GetOwner::<Impl, OFFSET>,
            GetSegmentData::<Impl, OFFSET>,
            GetSegmentTypes::<Impl, OFFSET>,
            GetSegmentStrokes::<Impl, OFFSET>,
            SetSegments::<Impl, OFFSET>,
            GetStartPoint::<Impl, OFFSET>,
            SetStartPoint::<Impl, OFFSET>,
            GetIsClosed::<Impl, OFFSET>,
            SetIsClosed::<Impl, OFFSET>,
            GetIsFilled::<Impl, OFFSET>,
            SetIsFilled::<Impl, OFFSET>,
            GetSegmentCount::<Impl, OFFSET>,
            GetSegmentDataCount::<Impl, OFFSET>,
            GetSegmentStrokePattern::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IXpsOMGeometryFigureCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGeometryFigureCollection";
}
impl IXpsOMGeometryFigureCollectionVtbl {
    pub const fn new<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGeometryFigureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&geometryfigure)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&geometryfigure as *const <IXpsOMGeometryFigure as ::windows::core::Abi>::Abi as *const <IXpsOMGeometryFigure as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&geometryfigure as *const <IXpsOMGeometryFigure as ::windows::core::Abi>::Abi as *const <IXpsOMGeometryFigure as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGeometryFigureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryfigure: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&geometryfigure as *const <IXpsOMGeometryFigure as ::windows::core::Abi>::Abi as *const <IXpsOMGeometryFigure as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMGeometryFigureCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMGlyphs {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGlyphs";
}
impl IXpsOMGlyphsVtbl {
    pub const fn new<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGlyphsVtbl {
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString(::core::mem::transmute_copy(&unicodestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount(::core::mem::transmute_copy(&indexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndices(indexcount, &*(&glyphindices as *const <XPS_GLYPH_INDEX as ::windows::core::Abi>::Abi as *const <XPS_GLYPH_INDEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount(::core::mem::transmute_copy(&glyphmappingcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappings(glyphmappingcount, &*(&glyphmappings as *const <XPS_GLYPH_MAPPING as ::windows::core::Abi>::Abi as *const <XPS_GLYPH_MAPPING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount(::core::mem::transmute_copy(&prohibitedcaretstopcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStops(prohibitedcaretstopcount, ::core::mem::transmute_copy(&prohibitedcaretstops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel(::core::mem::transmute_copy(&bidilevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsSideways(::core::mem::transmute_copy(&issideways)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName(::core::mem::transmute_copy(&devicefontname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStyleSimulations(::core::mem::transmute_copy(&stylesimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStyleSimulations(stylesimulations) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOrigin(::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrigin<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOrigin(&*(&origin as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontRenderingEmSize(::core::mem::transmute_copy(&fontrenderingemsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFontRenderingEmSize(fontrenderingemsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontResource(::core::mem::transmute_copy(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontResource<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFontResource(&*(&fontresource as *const <IXpsOMFontResource as ::windows::core::Abi>::Abi as *const <IXpsOMFontResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontFaceIndex(::core::mem::transmute_copy(&fontfaceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFontFaceIndex(fontfaceindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrush(::core::mem::transmute_copy(&fillbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal(::core::mem::transmute_copy(&fillbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillBrushLocal(&*(&fillbrush as *const <IXpsOMBrush as ::windows::core::Abi>::Abi as *const <IXpsOMBrush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillBrushLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphsEditor<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, editor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphsEditor(::core::mem::transmute_copy(&editor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGlyphsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&glyphs)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMGlyphs>,
            base.5,
            GetUnicodeString::<Impl, OFFSET>,
            GetGlyphIndexCount::<Impl, OFFSET>,
            GetGlyphIndices::<Impl, OFFSET>,
            GetGlyphMappingCount::<Impl, OFFSET>,
            GetGlyphMappings::<Impl, OFFSET>,
            GetProhibitedCaretStopCount::<Impl, OFFSET>,
            GetProhibitedCaretStops::<Impl, OFFSET>,
            GetBidiLevel::<Impl, OFFSET>,
            GetIsSideways::<Impl, OFFSET>,
            GetDeviceFontName::<Impl, OFFSET>,
            GetStyleSimulations::<Impl, OFFSET>,
            SetStyleSimulations::<Impl, OFFSET>,
            GetOrigin::<Impl, OFFSET>,
            SetOrigin::<Impl, OFFSET>,
            GetFontRenderingEmSize::<Impl, OFFSET>,
            SetFontRenderingEmSize::<Impl, OFFSET>,
            GetFontResource::<Impl, OFFSET>,
            SetFontResource::<Impl, OFFSET>,
            GetFontFaceIndex::<Impl, OFFSET>,
            SetFontFaceIndex::<Impl, OFFSET>,
            GetFillBrush::<Impl, OFFSET>,
            GetFillBrushLocal::<Impl, OFFSET>,
            SetFillBrushLocal::<Impl, OFFSET>,
            GetFillBrushLookup::<Impl, OFFSET>,
            SetFillBrushLookup::<Impl, OFFSET>,
            GetGlyphsEditor::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMGlyphsEditor {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGlyphsEditor";
}
impl IXpsOMGlyphsEditorVtbl {
    pub const fn new<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGlyphsEditorVtbl {
        unsafe extern "system" fn ApplyEdits<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplyEdits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUnicodeString(::core::mem::transmute_copy(&unicodestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeString<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUnicodeString(&*(&unicodestring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndexCount(::core::mem::transmute_copy(&indexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphIndices(indexcount, ::core::mem::transmute_copy(&glyphindices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlyphIndices<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGlyphIndices(indexcount, &*(&glyphindices as *const <XPS_GLYPH_INDEX as ::windows::core::Abi>::Abi as *const <XPS_GLYPH_INDEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappingCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappingCount(::core::mem::transmute_copy(&glyphmappingcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlyphMappings(glyphmappingcount, ::core::mem::transmute_copy(&glyphmappings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlyphMappings<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGlyphMappings(glyphmappingcount, &*(&glyphmappings as *const <XPS_GLYPH_MAPPING as ::windows::core::Abi>::Abi as *const <XPS_GLYPH_MAPPING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStopCount(::core::mem::transmute_copy(&prohibitedcaretstopcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProhibitedCaretStops(count, ::core::mem::transmute_copy(&prohibitedcaretstops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProhibitedCaretStops(count, prohibitedcaretstops) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBidiLevel(::core::mem::transmute_copy(&bidilevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBidiLevel<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBidiLevel(bidilevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsSideways(::core::mem::transmute_copy(&issideways)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSideways<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsSideways(&*(&issideways as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceFontName(::core::mem::transmute_copy(&devicefontname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Impl: IXpsOMGlyphsEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDeviceFontName(&*(&devicefontname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMGlyphsEditor>,
            base.5,
            ApplyEdits::<Impl, OFFSET>,
            GetUnicodeString::<Impl, OFFSET>,
            SetUnicodeString::<Impl, OFFSET>,
            GetGlyphIndexCount::<Impl, OFFSET>,
            GetGlyphIndices::<Impl, OFFSET>,
            SetGlyphIndices::<Impl, OFFSET>,
            GetGlyphMappingCount::<Impl, OFFSET>,
            GetGlyphMappings::<Impl, OFFSET>,
            SetGlyphMappings::<Impl, OFFSET>,
            GetProhibitedCaretStopCount::<Impl, OFFSET>,
            GetProhibitedCaretStops::<Impl, OFFSET>,
            SetProhibitedCaretStops::<Impl, OFFSET>,
            GetBidiLevel::<Impl, OFFSET>,
            SetBidiLevel::<Impl, OFFSET>,
            GetIsSideways::<Impl, OFFSET>,
            SetIsSideways::<Impl, OFFSET>,
            GetDeviceFontName::<Impl, OFFSET>,
            SetDeviceFontName::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMGradientBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGradientBrush";
}
impl IXpsOMGradientBrushVtbl {
    pub const fn new<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGradientBrushVtbl {
        unsafe extern "system" fn GetGradientStops<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGradientStops(::core::mem::transmute_copy(&gradientstops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLocal(&*(&transform as *const <IXpsOMMatrixTransform as ::windows::core::Abi>::Abi as *const <IXpsOMMatrixTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpreadMethod(::core::mem::transmute_copy(&spreadmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSpreadMethod(spreadmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorInterpolationMode(::core::mem::transmute_copy(&colorinterpolationmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Impl: IXpsOMGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorInterpolationMode(colorinterpolationmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMGradientBrush>, base.5, GetGradientStops::<Impl, OFFSET>, GetTransform::<Impl, OFFSET>, GetTransformLocal::<Impl, OFFSET>, SetTransformLocal::<Impl, OFFSET>, GetTransformLookup::<Impl, OFFSET>, SetTransformLookup::<Impl, OFFSET>, GetSpreadMethod::<Impl, OFFSET>, SetSpreadMethod::<Impl, OFFSET>, GetColorInterpolationMode::<Impl, OFFSET>, SetColorInterpolationMode::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IXpsOMGradientStop {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGradientStop";
}
impl IXpsOMGradientStopVtbl {
    pub const fn new<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGradientStopVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOffset(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOffset<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOffset(offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColor(&*(&color as *const <XPS_COLOR as ::windows::core::Abi>::Abi as *const <XPS_COLOR as ::windows::core::DefaultType>::DefaultType), &*(&colorprofile as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMGradientStopImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&gradientstop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMGradientStop>, base.5, GetOwner::<Impl, OFFSET>, GetOffset::<Impl, OFFSET>, SetOffset::<Impl, OFFSET>, GetColor::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IXpsOMGradientStopCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMGradientStopCollection";
}
impl IXpsOMGradientStopCollectionVtbl {
    pub const fn new<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMGradientStopCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&stop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&stop as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&stop as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMGradientStopCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&stop as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMGradientStopCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
pub trait IXpsOMImageBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetImageResource();
    fn SetImageResource();
    fn GetColorProfileResource();
    fn SetColorProfileResource();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMImageBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMImageBrush";
}
impl IXpsOMImageBrushVtbl {
    pub const fn new<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMImageBrushVtbl {
        unsafe extern "system" fn GetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageResource(::core::mem::transmute_copy(&imageresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageResource<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetImageResource(&*(&imageresource as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResource(::core::mem::transmute_copy(&colorprofileresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorProfileResource(&*(&colorprofileresource as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMImageBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&imagebrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMImageBrush>, base.5, GetImageResource::<Impl, OFFSET>, SetImageResource::<Impl, OFFSET>, GetColorProfileResource::<Impl, OFFSET>, SetColorProfileResource::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IXpsOMImageResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetImageType();
}
impl ::windows::core::RuntimeName for IXpsOMImageResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMImageResource";
}
impl IXpsOMImageResourceVtbl {
    pub const fn new<Impl: IXpsOMImageResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMImageResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMImageResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readerstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&readerstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMImageResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, imagetype: XPS_IMAGE_TYPE, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), imagetype, &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageType<Impl: IXpsOMImageResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageType(::core::mem::transmute_copy(&imagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMImageResource>, base.5, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>, GetImageType::<Impl, OFFSET>)
    }
}
pub trait IXpsOMImageResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
impl ::windows::core::RuntimeName for IXpsOMImageResourceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMImageResourceCollection";
}
impl IXpsOMImageResourceCollectionVtbl {
    pub const fn new<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMImageResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&object as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&object as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&object as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMImageResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByPartName(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&part)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMImageResourceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, GetByPartName::<Impl, OFFSET>)
    }
}
pub trait IXpsOMLinearGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetStartPoint();
    fn SetStartPoint();
    fn GetEndPoint();
    fn SetEndPoint();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMLinearGradientBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMLinearGradientBrush";
}
impl IXpsOMLinearGradientBrushVtbl {
    pub const fn new<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMLinearGradientBrushVtbl {
        unsafe extern "system" fn GetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStartPoint(::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStartPoint(&*(&startpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEndPoint(::core::mem::transmute_copy(&endpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPoint<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEndPoint(&*(&endpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMLinearGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&lineargradientbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMLinearGradientBrush>, base.5, GetStartPoint::<Impl, OFFSET>, SetStartPoint::<Impl, OFFSET>, GetEndPoint::<Impl, OFFSET>, SetEndPoint::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IXpsOMMatrixTransformImpl: Sized + IXpsOMShareableImpl {
    fn GetMatrix();
    fn SetMatrix();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMMatrixTransform {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMMatrixTransform";
}
impl IXpsOMMatrixTransformVtbl {
    pub const fn new<Impl: IXpsOMMatrixTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMMatrixTransformVtbl {
        unsafe extern "system" fn GetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMatrix(::core::mem::transmute_copy(&matrix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrix<Impl: IXpsOMMatrixTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMatrix(&*(&matrix as *const <XPS_MATRIX as ::windows::core::Abi>::Abi as *const <XPS_MATRIX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMMatrixTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&matrixtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMMatrixTransform>, base.5, GetMatrix::<Impl, OFFSET>, SetMatrix::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IXpsOMNameCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for IXpsOMNameCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMNameCollection";
}
impl IXpsOMNameCollectionVtbl {
    pub const fn new<Impl: IXpsOMNameCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMNameCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMNameCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMNameCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMNameCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMObjectFactory {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMObjectFactory";
}
impl IXpsOMObjectFactoryVtbl {
    pub const fn new<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMObjectFactoryVtbl {
        unsafe extern "system" fn CreatePackage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackage(::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream(&*(&stream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStoryFragmentsResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&storyfragmentsresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, documentstructureresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDocumentStructureResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&documentstructureresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSignatureBlockResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&signatureblockresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResource(&*(&dictionary as *const <IXpsOMDictionary as ::windows::core::Abi>::Abi as *const <IXpsOMDictionary as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&remotedictionaryresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, dictionaryparturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResourceFromStream(
                &*(&dictionarymarkupstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&dictionaryparturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&resources as *const <IXpsOMPartResources as ::windows::core::Abi>::Abi as *const <IXpsOMPartResources as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&dictionaryresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartResources<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePartResources(::core::mem::transmute_copy(&partresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDocumentSequence(&*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&documentsequence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocument<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDocument(&*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageReference<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePageReference(&*(&advisorypagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pagereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePage(
                &*(&pagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&page),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream(
                &*(&pagemarkupstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&resources as *const <IXpsOMPartResources as ::windows::core::Abi>::Abi as *const <IXpsOMPartResources as ::windows::core::DefaultType>::DefaultType),
                &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&page),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCanvas<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canvas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCanvas(::core::mem::transmute_copy(&canvas)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphs<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: ::windows::core::RawPtr, glyphs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGlyphs(&*(&fontresource as *const <IXpsOMFontResource as ::windows::core::Abi>::Abi as *const <IXpsOMFontResource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&glyphs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePath<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometry<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGeometry(::core::mem::transmute_copy(&geometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGeometryFigure(&*(&startpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&figure)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMatrixTransform(&*(&matrix as *const <XPS_MATRIX as ::windows::core::Abi>::Abi as *const <XPS_MATRIX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSolidColorBrush(&*(&color as *const <XPS_COLOR as ::windows::core::Abi>::Abi as *const <XPS_COLOR as ::windows::core::DefaultType>::DefaultType), &*(&colorprofile as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&solidcolorbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, colorprofileresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateColorProfileResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&colorprofileresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateImageBrush(&*(&image as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType), &*(&viewbox as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType), &*(&viewport as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&imagebrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVisualBrush(&*(&viewbox as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType), &*(&viewport as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&visualbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, contenttype: XPS_IMAGE_TYPE, parturi: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateImageResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), contenttype, &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&imageresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePrintTicketResource(&*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&printticketresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: ::windows::core::RawPtr, fontembedding: XPS_FONT_EMBEDDING, parturi: ::windows::core::RawPtr, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFontResource(
                &*(&acquiredstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                fontembedding,
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&isobfsourcestream as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&fontresource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStop<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr, offset: f32, gradientstop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGradientStop(&*(&color as *const <XPS_COLOR as ::windows::core::Abi>::Abi as *const <XPS_COLOR as ::windows::core::DefaultType>::DefaultType), &*(&colorprofile as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType), offset, ::core::mem::transmute_copy(&gradientstop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLinearGradientBrush(
                &*(&gradstop1 as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType),
                &*(&gradstop2 as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType),
                &*(&startpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType),
                &*(&endpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&lineargradientbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradstop1: ::windows::core::RawPtr, gradstop2: ::windows::core::RawPtr, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRadialGradientBrush(
                &*(&gradstop1 as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType),
                &*(&gradstop2 as *const <IXpsOMGradientStop as ::windows::core::Abi>::Abi as *const <IXpsOMGradientStop as ::windows::core::DefaultType>::DefaultType),
                &*(&centerpoint as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType),
                &*(&gradientorigin as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType),
                &*(&radiisizes as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&radialgradientbrush),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCoreProperties(&*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&coreproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDictionary<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDictionary(::core::mem::transmute_copy(&dictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturicollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePartUriCollection(::core::mem::transmute_copy(&parturicollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnFile(
                &*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&securityattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                flagsandattributes,
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                interleaving,
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&coreproperties as *const <IXpsOMCoreProperties as ::windows::core::Abi>::Abi as *const <IXpsOMCoreProperties as ::windows::core::DefaultType>::DefaultType),
                &*(&packagethumbnail as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType),
                &*(&documentsequenceprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream(
                &*(&outputstream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType),
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                interleaving,
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&coreproperties as *const <IXpsOMCoreProperties as ::windows::core::Abi>::Abi as *const <IXpsOMCoreProperties as ::windows::core::DefaultType>::DefaultType),
                &*(&packagethumbnail as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType),
                &*(&documentsequenceprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePartUri<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePartUri(&*(&uri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Impl: IXpsOMObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateReadOnlyStreamOnFile(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&stream)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMObjectFactory>,
            base.5,
            CreatePackage::<Impl, OFFSET>,
            CreatePackageFromFile::<Impl, OFFSET>,
            CreatePackageFromStream::<Impl, OFFSET>,
            CreateStoryFragmentsResource::<Impl, OFFSET>,
            CreateDocumentStructureResource::<Impl, OFFSET>,
            CreateSignatureBlockResource::<Impl, OFFSET>,
            CreateRemoteDictionaryResource::<Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream::<Impl, OFFSET>,
            CreatePartResources::<Impl, OFFSET>,
            CreateDocumentSequence::<Impl, OFFSET>,
            CreateDocument::<Impl, OFFSET>,
            CreatePageReference::<Impl, OFFSET>,
            CreatePage::<Impl, OFFSET>,
            CreatePageFromStream::<Impl, OFFSET>,
            CreateCanvas::<Impl, OFFSET>,
            CreateGlyphs::<Impl, OFFSET>,
            CreatePath::<Impl, OFFSET>,
            CreateGeometry::<Impl, OFFSET>,
            CreateGeometryFigure::<Impl, OFFSET>,
            CreateMatrixTransform::<Impl, OFFSET>,
            CreateSolidColorBrush::<Impl, OFFSET>,
            CreateColorProfileResource::<Impl, OFFSET>,
            CreateImageBrush::<Impl, OFFSET>,
            CreateVisualBrush::<Impl, OFFSET>,
            CreateImageResource::<Impl, OFFSET>,
            CreatePrintTicketResource::<Impl, OFFSET>,
            CreateFontResource::<Impl, OFFSET>,
            CreateGradientStop::<Impl, OFFSET>,
            CreateLinearGradientBrush::<Impl, OFFSET>,
            CreateRadialGradientBrush::<Impl, OFFSET>,
            CreateCoreProperties::<Impl, OFFSET>,
            CreateDictionary::<Impl, OFFSET>,
            CreatePartUriCollection::<Impl, OFFSET>,
            CreatePackageWriterOnFile::<Impl, OFFSET>,
            CreatePackageWriterOnStream::<Impl, OFFSET>,
            CreatePartUri::<Impl, OFFSET>,
            CreateReadOnlyStreamOnFile::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMObjectFactory1 {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMObjectFactory1";
}
impl IXpsOMObjectFactory1Vtbl {
    pub const fn new<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMObjectFactory1Vtbl {
        unsafe extern "system" fn GetDocumentTypeFromFile<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromFile(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: ::windows::core::RawPtr, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentTypeFromStream(&*(&xpsdocumentstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertHDPhotoToJpegXR(&*(&imageresource as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertJpegXRToHDPhoto(&*(&imageresource as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnFile1(
                &*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&securityattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                flagsandattributes,
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                interleaving,
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&coreproperties as *const <IXpsOMCoreProperties as ::windows::core::Abi>::Abi as *const <IXpsOMCoreProperties as ::windows::core::DefaultType>::DefaultType),
                &*(&packagethumbnail as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType),
                &*(&documentsequenceprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                documenttype,
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::windows::core::RawPtr, coreproperties: ::windows::core::RawPtr, packagethumbnail: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageWriterOnStream1(
                &*(&outputstream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType),
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                interleaving,
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&coreproperties as *const <IXpsOMCoreProperties as ::windows::core::Abi>::Abi as *const <IXpsOMCoreProperties as ::windows::core::DefaultType>::DefaultType),
                &*(&packagethumbnail as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType),
                &*(&documentsequenceprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                documenttype,
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackage1(::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromStream1(&*(&stream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePackageFromFile1(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePage1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: super::super::Foundation::PWSTR, parturi: ::windows::core::RawPtr, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePage1(
                &*(&pagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&page),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagemarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, reuseobjects: super::super::Foundation::BOOL, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePageFromStream1(
                &*(&pagemarkupstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&resources as *const <IXpsOMPartResources as ::windows::core::Abi>::Abi as *const <IXpsOMPartResources as ::windows::core::DefaultType>::DefaultType),
                &*(&reuseobjects as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&page),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Impl: IXpsOMObjectFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, resources: ::windows::core::RawPtr, dictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRemoteDictionaryResourceFromStream1(
                &*(&dictionarymarkupstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&resources as *const <IXpsOMPartResources as ::windows::core::Abi>::Abi as *const <IXpsOMPartResources as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&dictionaryresource),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMObjectFactory1>,
            base.5,
            GetDocumentTypeFromFile::<Impl, OFFSET>,
            GetDocumentTypeFromStream::<Impl, OFFSET>,
            ConvertHDPhotoToJpegXR::<Impl, OFFSET>,
            ConvertJpegXRToHDPhoto::<Impl, OFFSET>,
            CreatePackageWriterOnFile1::<Impl, OFFSET>,
            CreatePackageWriterOnStream1::<Impl, OFFSET>,
            CreatePackage1::<Impl, OFFSET>,
            CreatePackageFromStream1::<Impl, OFFSET>,
            CreatePackageFromFile1::<Impl, OFFSET>,
            CreatePage1::<Impl, OFFSET>,
            CreatePageFromStream1::<Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMPackage {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPackage";
}
impl IXpsOMPackageVtbl {
    pub const fn new<Impl: IXpsOMPackageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPackageVtbl {
        unsafe extern "system" fn GetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentSequence(::core::mem::transmute_copy(&documentsequence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDocumentSequence(&*(&documentsequence as *const <IXpsOMDocumentSequence as ::windows::core::Abi>::Abi as *const <IXpsOMDocumentSequence as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCoreProperties(::core::mem::transmute_copy(&coreproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoreProperties<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCoreProperties(&*(&coreproperties as *const <IXpsOMCoreProperties as ::windows::core::Abi>::Abi as *const <IXpsOMCoreProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDiscardControlPartName(::core::mem::transmute_copy(&discardcontrolparturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDiscardControlPartName(&*(&discardcontrolparturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource(::core::mem::transmute_copy(&imageresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetThumbnailResource(&*(&imageresource as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteToFile(
                &*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&securityattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                flagsandattributes,
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToStream<Impl: IXpsOMPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteToStream(&*(&stream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPackage>, base.5, GetDocumentSequence::<Impl, OFFSET>, SetDocumentSequence::<Impl, OFFSET>, GetCoreProperties::<Impl, OFFSET>, SetCoreProperties::<Impl, OFFSET>, GetDiscardControlPartName::<Impl, OFFSET>, SetDiscardControlPartName::<Impl, OFFSET>, GetThumbnailResource::<Impl, OFFSET>, SetThumbnailResource::<Impl, OFFSET>, WriteToFile::<Impl, OFFSET>, WriteToStream::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPackage1Impl: Sized + IXpsOMPackageImpl {
    fn GetDocumentType();
    fn WriteToFile1();
    fn WriteToStream1();
}
impl ::windows::core::RuntimeName for IXpsOMPackage1 {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPackage1";
}
impl IXpsOMPackage1Vtbl {
    pub const fn new<Impl: IXpsOMPackage1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPackage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPackage1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentType(::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToFile1<Impl: IXpsOMPackage1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteToFile1(
                &*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&securityattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                flagsandattributes,
                &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                documenttype,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToStream1<Impl: IXpsOMPackage1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteToStream1(&*(&outputstream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), documenttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPackage1>, base.5, GetDocumentType::<Impl, OFFSET>, WriteToFile1::<Impl, OFFSET>, WriteToStream1::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPackageTargetImpl: Sized {
    fn CreateXpsOMPackageWriter();
}
impl ::windows::core::RuntimeName for IXpsOMPackageTarget {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPackageTarget";
}
impl IXpsOMPackageTargetVtbl {
    pub const fn new<Impl: IXpsOMPackageTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPackageTargetVtbl {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Impl: IXpsOMPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: ::windows::core::RawPtr, documentsequenceprintticket: ::windows::core::RawPtr, discardcontrolpartname: ::windows::core::RawPtr, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateXpsOMPackageWriter(
                &*(&documentsequencepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&documentsequenceprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&discardcontrolpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&packagewriter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPackageTarget>, base.5, CreateXpsOMPackageWriter::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPackageWriterImpl: Sized {
    fn StartNewDocument();
    fn AddPage();
    fn AddResource();
    fn Close();
    fn IsClosed();
}
impl ::windows::core::RuntimeName for IXpsOMPackageWriter {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPackageWriter";
}
impl IXpsOMPackageWriterVtbl {
    pub const fn new<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPackageWriterVtbl {
        unsafe extern "system" fn StartNewDocument<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentpartname: ::windows::core::RawPtr, documentprintticket: ::windows::core::RawPtr, documentstructure: ::windows::core::RawPtr, signatureblockresources: ::windows::core::RawPtr, restrictedfonts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartNewDocument(
                &*(&documentpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType),
                &*(&documentprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&documentstructure as *const <IXpsOMDocumentStructureResource as ::windows::core::Abi>::Abi as *const <IXpsOMDocumentStructureResource as ::windows::core::DefaultType>::DefaultType),
                &*(&signatureblockresources as *const <IXpsOMSignatureBlockResourceCollection as ::windows::core::Abi>::Abi as *const <IXpsOMSignatureBlockResourceCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&restrictedfonts as *const <IXpsOMPartUriCollection as ::windows::core::Abi>::Abi as *const <IXpsOMPartUriCollection as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPage<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::windows::core::RawPtr, storyfragments: ::windows::core::RawPtr, pageprintticket: ::windows::core::RawPtr, pagethumbnail: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPage(
                &*(&page as *const <IXpsOMPage as ::windows::core::Abi>::Abi as *const <IXpsOMPage as ::windows::core::DefaultType>::DefaultType),
                &*(&advisorypagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType),
                &*(&discardableresourceparts as *const <IXpsOMPartUriCollection as ::windows::core::Abi>::Abi as *const <IXpsOMPartUriCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&storyfragments as *const <IXpsOMStoryFragmentsResource as ::windows::core::Abi>::Abi as *const <IXpsOMStoryFragmentsResource as ::windows::core::DefaultType>::DefaultType),
                &*(&pageprintticket as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType),
                &*(&pagethumbnail as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddResource<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddResource(&*(&resource as *const <IXpsOMResource as ::windows::core::Abi>::Abi as *const <IXpsOMResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClosed<Impl: IXpsOMPackageWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClosed(::core::mem::transmute_copy(&isclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPackageWriter>, base.5, StartNewDocument::<Impl, OFFSET>, AddPage::<Impl, OFFSET>, AddResource::<Impl, OFFSET>, Close::<Impl, OFFSET>, IsClosed::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPackageWriter3DImpl: Sized + IXpsOMPackageWriterImpl {
    fn AddModelTexture();
    fn SetModelPrintTicket();
}
impl ::windows::core::RuntimeName for IXpsOMPackageWriter3D {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPackageWriter3D";
}
impl IXpsOMPackageWriter3DVtbl {
    pub const fn new<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPackageWriter3DVtbl {
        unsafe extern "system" fn AddModelTexture<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, texturepartname: ::windows::core::RawPtr, texturedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddModelTexture(&*(&texturepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), &*(&texturedata as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelPrintTicket<Impl: IXpsOMPackageWriter3DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketpartname: ::windows::core::RawPtr, printticketdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetModelPrintTicket(&*(&printticketpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), &*(&printticketdata as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPackageWriter3D>, base.5, AddModelTexture::<Impl, OFFSET>, SetModelPrintTicket::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMPage {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPage";
}
impl IXpsOMPageVtbl {
    pub const fn new<Impl: IXpsOMPageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPageVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&pagereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisuals<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visuals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisuals(::core::mem::transmute_copy(&visuals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPageDimensions(::core::mem::transmute_copy(&pagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageDimensions<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPageDimensions(&*(&pagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentBox<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentBox(::core::mem::transmute_copy(&contentbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentBox<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContentBox(&*(&contentbox as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBleedBox(::core::mem::transmute_copy(&bleedbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBleedBox<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBleedBox(&*(&bleedbox as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguage(::core::mem::transmute_copy(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLanguage(&*(&language as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlinktarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsHyperlinkTarget(&*(&ishyperlinktarget as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionary(::core::mem::transmute_copy(&resourcedictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionaryLocal(::core::mem::transmute_copy(&resourcedictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDictionaryLocal(&*(&resourcedictionary as *const <IXpsOMDictionary as ::windows::core::Abi>::Abi as *const <IXpsOMDictionary as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionaryResource(::core::mem::transmute_copy(&remotedictionaryresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDictionaryResource(&*(&remotedictionaryresource as *const <IXpsOMRemoteDictionaryResource as ::windows::core::Abi>::Abi as *const <IXpsOMRemoteDictionaryResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write(&*(&stream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateUnusedLookupKey(r#type, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&page)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMPage>,
            base.5,
            GetOwner::<Impl, OFFSET>,
            GetVisuals::<Impl, OFFSET>,
            GetPageDimensions::<Impl, OFFSET>,
            SetPageDimensions::<Impl, OFFSET>,
            GetContentBox::<Impl, OFFSET>,
            SetContentBox::<Impl, OFFSET>,
            GetBleedBox::<Impl, OFFSET>,
            SetBleedBox::<Impl, OFFSET>,
            GetLanguage::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            GetIsHyperlinkTarget::<Impl, OFFSET>,
            SetIsHyperlinkTarget::<Impl, OFFSET>,
            GetDictionary::<Impl, OFFSET>,
            GetDictionaryLocal::<Impl, OFFSET>,
            SetDictionaryLocal::<Impl, OFFSET>,
            GetDictionaryResource::<Impl, OFFSET>,
            SetDictionaryResource::<Impl, OFFSET>,
            Write::<Impl, OFFSET>,
            GenerateUnusedLookupKey::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsOMPage1Impl: Sized + IXpsOMPageImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
impl ::windows::core::RuntimeName for IXpsOMPage1 {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPage1";
}
impl IXpsOMPage1Vtbl {
    pub const fn new<Impl: IXpsOMPage1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPage1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMPage1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentType(::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMPage1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write1(&*(&stream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), &*(&optimizemarkupsize as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), documenttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPage1>, base.5, GetDocumentType::<Impl, OFFSET>, Write1::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMPageReference {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPageReference";
}
impl IXpsOMPageReferenceVtbl {
    pub const fn new<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPageReferenceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&document)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPage(::core::mem::transmute_copy(&page)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPage(&*(&page as *const <IXpsOMPage as ::windows::core::Abi>::Abi as *const <IXpsOMPage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardPage<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiscardPage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPageLoaded<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPageLoaded(::core::mem::transmute_copy(&ispageloaded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdvisoryPageDimensions(::core::mem::transmute_copy(&pagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAdvisoryPageDimensions(&*(&pagedimensions as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStoryFragmentsResource(::core::mem::transmute_copy(&storyfragmentsresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStoryFragmentsResource(&*(&storyfragmentsresource as *const <IXpsOMStoryFragmentsResource as ::windows::core::Abi>::Abi as *const <IXpsOMStoryFragmentsResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintTicketResource(::core::mem::transmute_copy(&printticketresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrintTicketResource(&*(&printticketresource as *const <IXpsOMPrintTicketResource as ::windows::core::Abi>::Abi as *const <IXpsOMPrintTicketResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetThumbnailResource(::core::mem::transmute_copy(&imageresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetThumbnailResource(&*(&imageresource as *const <IXpsOMImageResource as ::windows::core::Abi>::Abi as *const <IXpsOMImageResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectLinkTargets<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linktargets: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollectLinkTargets(::core::mem::transmute_copy(&linktargets)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectPartResources<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollectPartResources(::core::mem::transmute_copy(&partresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasRestrictedFonts(::core::mem::transmute_copy(&restrictedfonts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPageReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pagereference)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMPageReference>,
            base.5,
            GetOwner::<Impl, OFFSET>,
            GetPage::<Impl, OFFSET>,
            SetPage::<Impl, OFFSET>,
            DiscardPage::<Impl, OFFSET>,
            IsPageLoaded::<Impl, OFFSET>,
            GetAdvisoryPageDimensions::<Impl, OFFSET>,
            SetAdvisoryPageDimensions::<Impl, OFFSET>,
            GetStoryFragmentsResource::<Impl, OFFSET>,
            SetStoryFragmentsResource::<Impl, OFFSET>,
            GetPrintTicketResource::<Impl, OFFSET>,
            SetPrintTicketResource::<Impl, OFFSET>,
            GetThumbnailResource::<Impl, OFFSET>,
            SetThumbnailResource::<Impl, OFFSET>,
            CollectLinkTargets::<Impl, OFFSET>,
            CollectPartResources::<Impl, OFFSET>,
            HasRestrictedFonts::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IXpsOMPageReferenceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPageReferenceCollection";
}
impl IXpsOMPageReferenceCollectionVtbl {
    pub const fn new<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPageReferenceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&pagereference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&pagereference as *const <IXpsOMPageReference as ::windows::core::Abi>::Abi as *const <IXpsOMPageReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&pagereference as *const <IXpsOMPageReference as ::windows::core::Abi>::Abi as *const <IXpsOMPageReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPageReferenceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&pagereference as *const <IXpsOMPageReference as ::windows::core::Abi>::Abi as *const <IXpsOMPageReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPageReferenceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPartImpl: Sized {
    fn GetPartName();
    fn SetPartName();
}
impl ::windows::core::RuntimeName for IXpsOMPart {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPart";
}
impl IXpsOMPartVtbl {
    pub const fn new<Impl: IXpsOMPartImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPartVtbl {
        unsafe extern "system" fn GetPartName<Impl: IXpsOMPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartName(::core::mem::transmute_copy(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartName<Impl: IXpsOMPartImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPartName(&*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPart>, base.5, GetPartName::<Impl, OFFSET>, SetPartName::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPartResourcesImpl: Sized {
    fn GetFontResources();
    fn GetImageResources();
    fn GetColorProfileResources();
    fn GetRemoteDictionaryResources();
}
impl ::windows::core::RuntimeName for IXpsOMPartResources {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPartResources";
}
impl IXpsOMPartResourcesVtbl {
    pub const fn new<Impl: IXpsOMPartResourcesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPartResourcesVtbl {
        unsafe extern "system" fn GetFontResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontResources(::core::mem::transmute_copy(&fontresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageResources(::core::mem::transmute_copy(&imageresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColorProfileResources(::core::mem::transmute_copy(&colorprofileresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Impl: IXpsOMPartResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRemoteDictionaryResources(::core::mem::transmute_copy(&dictionaryresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPartResources>, base.5, GetFontResources::<Impl, OFFSET>, GetImageResources::<Impl, OFFSET>, GetColorProfileResources::<Impl, OFFSET>, GetRemoteDictionaryResources::<Impl, OFFSET>)
    }
}
pub trait IXpsOMPartUriCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
impl ::windows::core::RuntimeName for IXpsOMPartUriCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPartUriCollection";
}
impl IXpsOMPartUriCollectionVtbl {
    pub const fn new<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPartUriCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMPartUriCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&parturi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPartUriCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMPath {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPath";
}
impl IXpsOMPathVtbl {
    pub const fn new<Impl: IXpsOMPathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPathVtbl {
        unsafe extern "system" fn GetGeometry<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeometry(::core::mem::transmute_copy(&geometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeometryLocal(::core::mem::transmute_copy(&geometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGeometryLocal(&*(&geometry as *const <IXpsOMGeometry as ::windows::core::Abi>::Abi as *const <IXpsOMGeometry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeometryLookup(::core::mem::transmute_copy(&lookup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGeometryLookup(&*(&lookup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityShortDescription(::core::mem::transmute_copy(&shortdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccessibilityShortDescription(&*(&shortdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessibilityLongDescription(::core::mem::transmute_copy(&longdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccessibilityLongDescription(&*(&longdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSnapsToPixels(::core::mem::transmute_copy(&snapstopixels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSnapsToPixels(&*(&snapstopixels as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrush<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrush(::core::mem::transmute_copy(&brush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLocal(::core::mem::transmute_copy(&brush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeBrushLocal(&*(&brush as *const <IXpsOMBrush as ::windows::core::Abi>::Abi as *const <IXpsOMBrush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeBrushLookup(::core::mem::transmute_copy(&lookup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeBrushLookup(&*(&lookup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashes<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashes(::core::mem::transmute_copy(&strokedashes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashCap(::core::mem::transmute_copy(&strokedashcap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeDashCap(strokedashcap) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeDashOffset(::core::mem::transmute_copy(&strokedashoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeDashOffset(strokedashoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeStartLineCap(::core::mem::transmute_copy(&strokestartlinecap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeStartLineCap(strokestartlinecap) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeEndLineCap(::core::mem::transmute_copy(&strokeendlinecap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeEndLineCap(strokeendlinecap) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeLineJoin(::core::mem::transmute_copy(&strokelinejoin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeLineJoin(strokelinejoin) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeMiterLimit(::core::mem::transmute_copy(&strokemiterlimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeMiterLimit(strokemiterlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStrokeThickness(::core::mem::transmute_copy(&strokethickness)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStrokeThickness(strokethickness) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrush<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrush(::core::mem::transmute_copy(&brush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLocal(::core::mem::transmute_copy(&brush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillBrushLocal(&*(&brush as *const <IXpsOMBrush as ::windows::core::Abi>::Abi as *const <IXpsOMBrush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFillBrushLookup(::core::mem::transmute_copy(&lookup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFillBrushLookup(&*(&lookup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&path)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMPath>,
            base.5,
            GetGeometry::<Impl, OFFSET>,
            GetGeometryLocal::<Impl, OFFSET>,
            SetGeometryLocal::<Impl, OFFSET>,
            GetGeometryLookup::<Impl, OFFSET>,
            SetGeometryLookup::<Impl, OFFSET>,
            GetAccessibilityShortDescription::<Impl, OFFSET>,
            SetAccessibilityShortDescription::<Impl, OFFSET>,
            GetAccessibilityLongDescription::<Impl, OFFSET>,
            SetAccessibilityLongDescription::<Impl, OFFSET>,
            GetSnapsToPixels::<Impl, OFFSET>,
            SetSnapsToPixels::<Impl, OFFSET>,
            GetStrokeBrush::<Impl, OFFSET>,
            GetStrokeBrushLocal::<Impl, OFFSET>,
            SetStrokeBrushLocal::<Impl, OFFSET>,
            GetStrokeBrushLookup::<Impl, OFFSET>,
            SetStrokeBrushLookup::<Impl, OFFSET>,
            GetStrokeDashes::<Impl, OFFSET>,
            GetStrokeDashCap::<Impl, OFFSET>,
            SetStrokeDashCap::<Impl, OFFSET>,
            GetStrokeDashOffset::<Impl, OFFSET>,
            SetStrokeDashOffset::<Impl, OFFSET>,
            GetStrokeStartLineCap::<Impl, OFFSET>,
            SetStrokeStartLineCap::<Impl, OFFSET>,
            GetStrokeEndLineCap::<Impl, OFFSET>,
            SetStrokeEndLineCap::<Impl, OFFSET>,
            GetStrokeLineJoin::<Impl, OFFSET>,
            SetStrokeLineJoin::<Impl, OFFSET>,
            GetStrokeMiterLimit::<Impl, OFFSET>,
            SetStrokeMiterLimit::<Impl, OFFSET>,
            GetStrokeThickness::<Impl, OFFSET>,
            SetStrokeThickness::<Impl, OFFSET>,
            GetFillBrush::<Impl, OFFSET>,
            GetFillBrushLocal::<Impl, OFFSET>,
            SetFillBrushLocal::<Impl, OFFSET>,
            GetFillBrushLookup::<Impl, OFFSET>,
            SetFillBrushLookup::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsOMPrintTicketResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
impl ::windows::core::RuntimeName for IXpsOMPrintTicketResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMPrintTicketResource";
}
impl IXpsOMPrintTicketResourceVtbl {
    pub const fn new<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMPrintTicketResourceVtbl {
        unsafe extern "system" fn GetStream<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMPrintTicketResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMPrintTicketResource>, base.5, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>)
    }
}
pub trait IXpsOMRadialGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetCenter();
    fn SetCenter();
    fn GetRadiiSizes();
    fn SetRadiiSizes();
    fn GetGradientOrigin();
    fn SetGradientOrigin();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMRadialGradientBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMRadialGradientBrush";
}
impl IXpsOMRadialGradientBrushVtbl {
    pub const fn new<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMRadialGradientBrushVtbl {
        unsafe extern "system" fn GetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCenter(::core::mem::transmute_copy(&center)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCenter(&*(&center as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRadiiSizes(::core::mem::transmute_copy(&radiisizes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRadiiSizes(&*(&radiisizes as *const <XPS_SIZE as ::windows::core::Abi>::Abi as *const <XPS_SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGradientOrigin(::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetGradientOrigin(&*(&origin as *const <XPS_POINT as ::windows::core::Abi>::Abi as *const <XPS_POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMRadialGradientBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&radialgradientbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMRadialGradientBrush>, base.5, GetCenter::<Impl, OFFSET>, SetCenter::<Impl, OFFSET>, GetRadiiSizes::<Impl, OFFSET>, SetRadiiSizes::<Impl, OFFSET>, GetGradientOrigin::<Impl, OFFSET>, SetGradientOrigin::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IXpsOMRemoteDictionaryResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDictionary();
    fn SetDictionary();
}
impl ::windows::core::RuntimeName for IXpsOMRemoteDictionaryResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMRemoteDictionaryResource";
}
impl IXpsOMRemoteDictionaryResourceVtbl {
    pub const fn new<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMRemoteDictionaryResourceVtbl {
        unsafe extern "system" fn GetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDictionary(::core::mem::transmute_copy(&dictionary)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDictionary<Impl: IXpsOMRemoteDictionaryResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDictionary(&*(&dictionary as *const <IXpsOMDictionary as ::windows::core::Abi>::Abi as *const <IXpsOMDictionary as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMRemoteDictionaryResource>, base.5, GetDictionary::<Impl, OFFSET>, SetDictionary::<Impl, OFFSET>)
    }
}
pub trait IXpsOMRemoteDictionaryResource1Impl: Sized + IXpsOMRemoteDictionaryResourceImpl + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
impl ::windows::core::RuntimeName for IXpsOMRemoteDictionaryResource1 {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMRemoteDictionaryResource1";
}
impl IXpsOMRemoteDictionaryResource1Vtbl {
    pub const fn new<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMRemoteDictionaryResource1Vtbl {
        unsafe extern "system" fn GetDocumentType<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentType(::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write1<Impl: IXpsOMRemoteDictionaryResource1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write1(&*(&stream as *const <super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), documenttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMRemoteDictionaryResource1>, base.5, GetDocumentType::<Impl, OFFSET>, Write1::<Impl, OFFSET>)
    }
}
pub trait IXpsOMRemoteDictionaryResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
impl ::windows::core::RuntimeName for IXpsOMRemoteDictionaryResourceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMRemoteDictionaryResourceCollection";
}
impl IXpsOMRemoteDictionaryResourceCollectionVtbl {
    pub const fn new<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMRemoteDictionaryResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&object as *const <IXpsOMRemoteDictionaryResource as ::windows::core::Abi>::Abi as *const <IXpsOMRemoteDictionaryResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&object as *const <IXpsOMRemoteDictionaryResource as ::windows::core::Abi>::Abi as *const <IXpsOMRemoteDictionaryResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&object as *const <IXpsOMRemoteDictionaryResource as ::windows::core::Abi>::Abi as *const <IXpsOMRemoteDictionaryResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMRemoteDictionaryResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, remotedictionaryresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByPartName(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&remotedictionaryresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMRemoteDictionaryResourceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, GetByPartName::<Impl, OFFSET>)
    }
}
pub trait IXpsOMResourceImpl: Sized + IXpsOMPartImpl {}
impl ::windows::core::RuntimeName for IXpsOMResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMResource";
}
impl IXpsOMResourceVtbl {
    pub const fn new<Impl: IXpsOMResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMResourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMResource>, base.5)
    }
}
pub trait IXpsOMShareableImpl: Sized {
    fn GetOwner();
    fn GetType();
}
impl ::windows::core::RuntimeName for IXpsOMShareable {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMShareable";
}
impl IXpsOMShareableVtbl {
    pub const fn new<Impl: IXpsOMShareableImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMShareableVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMShareableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IXpsOMShareableImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMShareable>, base.5, GetOwner::<Impl, OFFSET>, GetType::<Impl, OFFSET>)
    }
}
pub trait IXpsOMSignatureBlockResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
impl ::windows::core::RuntimeName for IXpsOMSignatureBlockResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMSignatureBlockResource";
}
impl IXpsOMSignatureBlockResourceVtbl {
    pub const fn new<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMSignatureBlockResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMSignatureBlockResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMSignatureBlockResource>, base.5, GetOwner::<Impl, OFFSET>, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>)
    }
}
pub trait IXpsOMSignatureBlockResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
impl ::windows::core::RuntimeName for IXpsOMSignatureBlockResourceCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMSignatureBlockResourceCollection";
}
impl IXpsOMSignatureBlockResourceCollectionVtbl {
    pub const fn new<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMSignatureBlockResourceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&signatureblockresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&signatureblockresource as *const <IXpsOMSignatureBlockResource as ::windows::core::Abi>::Abi as *const <IXpsOMSignatureBlockResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&signatureblockresource as *const <IXpsOMSignatureBlockResource as ::windows::core::Abi>::Abi as *const <IXpsOMSignatureBlockResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblockresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&signatureblockresource as *const <IXpsOMSignatureBlockResource as ::windows::core::Abi>::Abi as *const <IXpsOMSignatureBlockResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByPartName<Impl: IXpsOMSignatureBlockResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, signatureblockresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetByPartName(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&signatureblockresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMSignatureBlockResourceCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, GetByPartName::<Impl, OFFSET>)
    }
}
pub trait IXpsOMSolidColorBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetColor();
    fn SetColor();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMSolidColorBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMSolidColorBrush";
}
impl IXpsOMSolidColorBrushVtbl {
    pub const fn new<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMSolidColorBrushVtbl {
        unsafe extern "system" fn GetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColor(::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColor(&*(&color as *const <XPS_COLOR as ::windows::core::Abi>::Abi as *const <XPS_COLOR as ::windows::core::DefaultType>::DefaultType), &*(&colorprofile as *const <IXpsOMColorProfileResource as ::windows::core::Abi>::Abi as *const <IXpsOMColorProfileResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMSolidColorBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&solidcolorbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMSolidColorBrush>, base.5, GetColor::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IXpsOMStoryFragmentsResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
impl ::windows::core::RuntimeName for IXpsOMStoryFragmentsResource {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMStoryFragmentsResource";
}
impl IXpsOMStoryFragmentsResourceVtbl {
    pub const fn new<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMStoryFragmentsResourceVtbl {
        unsafe extern "system" fn GetOwner<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&owner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IXpsOMStoryFragmentsResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: ::windows::core::RawPtr, partname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContent(&*(&sourcestream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMStoryFragmentsResource>, base.5, GetOwner::<Impl, OFFSET>, GetStream::<Impl, OFFSET>, SetContent::<Impl, OFFSET>)
    }
}
pub trait IXpsOMThumbnailGeneratorImpl: Sized {
    fn GenerateThumbnail();
}
impl ::windows::core::RuntimeName for IXpsOMThumbnailGenerator {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMThumbnailGenerator";
}
impl IXpsOMThumbnailGeneratorVtbl {
    pub const fn new<Impl: IXpsOMThumbnailGeneratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMThumbnailGeneratorVtbl {
        unsafe extern "system" fn GenerateThumbnail<Impl: IXpsOMThumbnailGeneratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: ::windows::core::RawPtr, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::windows::core::RawPtr, imageresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateThumbnail(&*(&page as *const <IXpsOMPage as ::windows::core::Abi>::Abi as *const <IXpsOMPage as ::windows::core::DefaultType>::DefaultType), thumbnailtype, thumbnailsize, &*(&imageresourcepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&imageresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMThumbnailGenerator>, base.5, GenerateThumbnail::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMTileBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMTileBrush";
}
impl IXpsOMTileBrushVtbl {
    pub const fn new<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMTileBrushVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal(::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLocal(&*(&transform as *const <IXpsOMMatrixTransform as ::windows::core::Abi>::Abi as *const <IXpsOMMatrixTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewbox(::core::mem::transmute_copy(&viewbox)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewbox<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewbox(&*(&viewbox as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewport(::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewport<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewport(&*(&viewport as *const <XPS_RECT as ::windows::core::Abi>::Abi as *const <XPS_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTileMode(::core::mem::transmute_copy(&tilemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileMode<Impl: IXpsOMTileBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTileMode(tilemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMTileBrush>, base.5, GetTransform::<Impl, OFFSET>, GetTransformLocal::<Impl, OFFSET>, SetTransformLocal::<Impl, OFFSET>, GetTransformLookup::<Impl, OFFSET>, SetTransformLookup::<Impl, OFFSET>, GetViewbox::<Impl, OFFSET>, SetViewbox::<Impl, OFFSET>, GetViewport::<Impl, OFFSET>, SetViewport::<Impl, OFFSET>, GetTileMode::<Impl, OFFSET>, SetTileMode::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsOMVisual {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMVisual";
}
impl IXpsOMVisualVtbl {
    pub const fn new<Impl: IXpsOMVisualImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMVisualVtbl {
        unsafe extern "system" fn GetTransform<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransform(::core::mem::transmute_copy(&matrixtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLocal(::core::mem::transmute_copy(&matrixtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLocal(&*(&matrixtransform as *const <IXpsOMMatrixTransform as ::windows::core::Abi>::Abi as *const <IXpsOMMatrixTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransformLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometry<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipGeometry(::core::mem::transmute_copy(&clipgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLocal(::core::mem::transmute_copy(&clipgeometry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipGeometryLocal(&*(&clipgeometry as *const <IXpsOMGeometry as ::windows::core::Abi>::Abi as *const <IXpsOMGeometry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClipGeometryLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClipGeometryLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacity(::core::mem::transmute_copy(&opacity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOpacity(opacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrush(::core::mem::transmute_copy(&opacitymaskbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLocal(::core::mem::transmute_copy(&opacitymaskbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOpacityMaskBrushLocal(&*(&opacitymaskbrush as *const <IXpsOMBrush as ::windows::core::Abi>::Abi as *const <IXpsOMBrush as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOpacityMaskBrushLookup(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOpacityMaskBrushLookup(&*(&key as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsHyperlinkTarget(::core::mem::transmute_copy(&ishyperlink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsHyperlinkTarget(&*(&ishyperlink as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHyperlinkNavigateUri(::core::mem::transmute_copy(&hyperlinkuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hyperlinkuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHyperlinkNavigateUri(&*(&hyperlinkuri as *const <super::super::System::Com::IUri as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguage(::core::mem::transmute_copy(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IXpsOMVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLanguage(&*(&language as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsOMVisual>,
            base.5,
            GetTransform::<Impl, OFFSET>,
            GetTransformLocal::<Impl, OFFSET>,
            SetTransformLocal::<Impl, OFFSET>,
            GetTransformLookup::<Impl, OFFSET>,
            SetTransformLookup::<Impl, OFFSET>,
            GetClipGeometry::<Impl, OFFSET>,
            GetClipGeometryLocal::<Impl, OFFSET>,
            SetClipGeometryLocal::<Impl, OFFSET>,
            GetClipGeometryLookup::<Impl, OFFSET>,
            SetClipGeometryLookup::<Impl, OFFSET>,
            GetOpacity::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            GetOpacityMaskBrush::<Impl, OFFSET>,
            GetOpacityMaskBrushLocal::<Impl, OFFSET>,
            SetOpacityMaskBrushLocal::<Impl, OFFSET>,
            GetOpacityMaskBrushLookup::<Impl, OFFSET>,
            SetOpacityMaskBrushLookup::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            GetIsHyperlinkTarget::<Impl, OFFSET>,
            SetIsHyperlinkTarget::<Impl, OFFSET>,
            GetHyperlinkNavigateUri::<Impl, OFFSET>,
            SetHyperlinkNavigateUri::<Impl, OFFSET>,
            GetLanguage::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsOMVisualBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetVisual();
    fn GetVisualLocal();
    fn SetVisualLocal();
    fn GetVisualLookup();
    fn SetVisualLookup();
    fn Clone();
}
impl ::windows::core::RuntimeName for IXpsOMVisualBrush {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMVisualBrush";
}
impl IXpsOMVisualBrushVtbl {
    pub const fn new<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMVisualBrushVtbl {
        unsafe extern "system" fn GetVisual<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisual(::core::mem::transmute_copy(&visual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisualLocal(::core::mem::transmute_copy(&visual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLocal<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVisualLocal(&*(&visual as *const <IXpsOMVisual as ::windows::core::Abi>::Abi as *const <IXpsOMVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisualLookup(::core::mem::transmute_copy(&lookup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisualLookup<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVisualLookup(&*(&lookup as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IXpsOMVisualBrushImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visualbrush: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&visualbrush)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMVisualBrush>, base.5, GetVisual::<Impl, OFFSET>, GetVisualLocal::<Impl, OFFSET>, SetVisualLocal::<Impl, OFFSET>, GetVisualLookup::<Impl, OFFSET>, SetVisualLookup::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IXpsOMVisualCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsOMVisualCollection";
}
impl IXpsOMVisualCollectionVtbl {
    pub const fn new<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsOMVisualCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(index, &*(&object as *const <IXpsOMVisual as ::windows::core::Abi>::Abi as *const <IXpsOMVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAt(index, &*(&object as *const <IXpsOMVisual as ::windows::core::Abi>::Abi as *const <IXpsOMVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IXpsOMVisualCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&object as *const <IXpsOMVisual as ::windows::core::Abi>::Abi as *const <IXpsOMVisual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsOMVisualCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, SetAt::<Impl, OFFSET>, Append::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsSignature {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignature";
}
impl IXpsSignatureVtbl {
    pub const fn new<Impl: IXpsSignatureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sigid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureId(::core::mem::transmute_copy(&sigid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureValue<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureValue(::core::mem::transmute_copy(&signaturehashvalue), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateEnumerator(::core::mem::transmute_copy(&certificateenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTime<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSigningTime(::core::mem::transmute_copy(&sigdatetimestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat(::core::mem::transmute_copy(&timeformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName(::core::mem::transmute_copy(&signaturepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verify<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Verify(&*(&x509certificate as *const <super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&sigstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPolicy(::core::mem::transmute_copy(&policy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomObjectEnumerator(::core::mem::transmute_copy(&customobjectenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomReferenceEnumerator(::core::mem::transmute_copy(&customreferenceenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureXml(::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureXml<Impl: IXpsSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureXml(signaturexml, count) {
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
            ::windows::core::GetRuntimeClassName::<IXpsSignature>,
            base.5,
            GetSignatureId::<Impl, OFFSET>,
            GetSignatureValue::<Impl, OFFSET>,
            GetCertificateEnumerator::<Impl, OFFSET>,
            GetSigningTime::<Impl, OFFSET>,
            GetSigningTimeFormat::<Impl, OFFSET>,
            GetSignaturePartName::<Impl, OFFSET>,
            Verify::<Impl, OFFSET>,
            GetPolicy::<Impl, OFFSET>,
            GetCustomObjectEnumerator::<Impl, OFFSET>,
            GetCustomReferenceEnumerator::<Impl, OFFSET>,
            GetSignatureXml::<Impl, OFFSET>,
            SetSignatureXml::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsSignatureBlockImpl: Sized {
    fn GetRequests();
    fn GetPartName();
    fn GetDocumentIndex();
    fn GetDocumentName();
    fn CreateRequest();
}
impl ::windows::core::RuntimeName for IXpsSignatureBlock {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureBlock";
}
impl IXpsSignatureBlockVtbl {
    pub const fn new<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureBlockVtbl {
        unsafe extern "system" fn GetRequests<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requests: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequests(::core::mem::transmute_copy(&requests)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartName<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPartName(::core::mem::transmute_copy(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentIndex(::core::mem::transmute_copy(&fixeddocumentindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentName<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDocumentName(::core::mem::transmute_copy(&fixeddocumentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRequest<Impl: IXpsSignatureBlockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: super::super::Foundation::PWSTR, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRequest(&*(&requestid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&signaturerequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsSignatureBlock>, base.5, GetRequests::<Impl, OFFSET>, GetPartName::<Impl, OFFSET>, GetDocumentIndex::<Impl, OFFSET>, GetDocumentName::<Impl, OFFSET>, CreateRequest::<Impl, OFFSET>)
    }
}
pub trait IXpsSignatureBlockCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IXpsSignatureBlockCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureBlockCollection";
}
impl IXpsSignatureBlockCollectionVtbl {
    pub const fn new<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureBlockCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&signatureblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureBlockCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsSignatureBlockCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
pub trait IXpsSignatureCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IXpsSignatureCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureCollection";
}
impl IXpsSignatureCollectionVtbl {
    pub const fn new<Impl: IXpsSignatureCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&signature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsSignatureCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsSignatureManager {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureManager";
}
impl IXpsSignatureManagerVtbl {
    pub const fn new<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureManagerVtbl {
        unsafe extern "system" fn LoadPackageFile<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadPackageFile(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadPackageStream<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadPackageStream(&*(&stream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sign<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signoptions: ::windows::core::RawPtr, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Sign(&*(&signoptions as *const <IXpsSigningOptions as ::windows::core::Abi>::Abi as *const <IXpsSigningOptions as ::windows::core::DefaultType>::DefaultType), &*(&x509certificate as *const <super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::CERT_CONTEXT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&signature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureOriginPartName(::core::mem::transmute_copy(&signatureoriginpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureOriginPartName(&*(&signatureoriginpartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatures<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatures: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatures(::core::mem::transmute_copy(&signatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: ::windows::core::RawPtr, fixeddocumentindex: u32, signatureblock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSignatureBlock(&*(&partname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType), fixeddocumentindex, ::core::mem::transmute_copy(&signatureblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureBlocks(::core::mem::transmute_copy(&signatureblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSigningOptions(::core::mem::transmute_copy(&signingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToFile<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SavePackageToFile(&*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&securityattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType), flagsandattributes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePackageToStream<Impl: IXpsSignatureManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SavePackageToStream(&*(&stream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsSignatureManager>,
            base.5,
            LoadPackageFile::<Impl, OFFSET>,
            LoadPackageStream::<Impl, OFFSET>,
            Sign::<Impl, OFFSET>,
            GetSignatureOriginPartName::<Impl, OFFSET>,
            SetSignatureOriginPartName::<Impl, OFFSET>,
            GetSignatures::<Impl, OFFSET>,
            AddSignatureBlock::<Impl, OFFSET>,
            GetSignatureBlocks::<Impl, OFFSET>,
            CreateSigningOptions::<Impl, OFFSET>,
            SavePackageToFile::<Impl, OFFSET>,
            SavePackageToStream::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IXpsSignatureRequest {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureRequest";
}
impl IXpsSignatureRequestVtbl {
    pub const fn new<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureRequestVtbl {
        unsafe extern "system" fn GetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intent: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntent(::core::mem::transmute_copy(&intent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntent<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intent: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIntent(&*(&intent as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequestedSigner(::core::mem::transmute_copy(&signername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRequestedSigner(&*(&signername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datestring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequestSignByDate(::core::mem::transmute_copy(&datestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRequestSignByDate(&*(&datestring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, place: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSigningLocale(::core::mem::transmute_copy(&place)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningLocale<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, place: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSigningLocale(&*(&place as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut ::windows::core::RawPtr, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpotLocation(::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&pagepartname), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpotLocation<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSpotLocation(pageindex, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestId<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequestId(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IXpsSignatureRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignature(::core::mem::transmute_copy(&signature)) {
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
            ::windows::core::GetRuntimeClassName::<IXpsSignatureRequest>,
            base.5,
            GetIntent::<Impl, OFFSET>,
            SetIntent::<Impl, OFFSET>,
            GetRequestedSigner::<Impl, OFFSET>,
            SetRequestedSigner::<Impl, OFFSET>,
            GetRequestSignByDate::<Impl, OFFSET>,
            SetRequestSignByDate::<Impl, OFFSET>,
            GetSigningLocale::<Impl, OFFSET>,
            SetSigningLocale::<Impl, OFFSET>,
            GetSpotLocation::<Impl, OFFSET>,
            SetSpotLocation::<Impl, OFFSET>,
            GetRequestId::<Impl, OFFSET>,
            GetSignature::<Impl, OFFSET>,
        )
    }
}
pub trait IXpsSignatureRequestCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IXpsSignatureRequestCollection {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSignatureRequestCollection";
}
impl IXpsSignatureRequestCollectionVtbl {
    pub const fn new<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSignatureRequestCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(index, ::core::mem::transmute_copy(&signaturerequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IXpsSignatureRequestCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsSignatureRequestCollection>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXpsSigningOptions {
    const NAME: &'static str = "Windows.Win32.Storage.Xps.IXpsSigningOptions";
}
impl IXpsSigningOptionsVtbl {
    pub const fn new<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsSigningOptionsVtbl {
        unsafe extern "system" fn GetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureId(::core::mem::transmute_copy(&signatureid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureId<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureId(&*(&signatureid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignatureMethod(::core::mem::transmute_copy(&signaturemethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignatureMethod(&*(&signaturemethod as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDigestMethod(::core::mem::transmute_copy(&digestmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigestMethod<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDigestMethod(&*(&digestmethod as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignaturePartName(::core::mem::transmute_copy(&signaturepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSignaturePartName(&*(&signaturepartname as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::Abi>::Abi as *const <super::Packaging::Opc::IOpcPartUri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPolicy(::core::mem::transmute_copy(&policy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolicy<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPolicy(policy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSigningTimeFormat(::core::mem::transmute_copy(&timeformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSigningTimeFormat(timeformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomObjects<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomObjects(::core::mem::transmute_copy(&customobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomReferences<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomReferences(::core::mem::transmute_copy(&customreferenceset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateSet<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateSet(::core::mem::transmute_copy(&certificateset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IXpsSigningOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFlags(flags) {
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
            ::windows::core::GetRuntimeClassName::<IXpsSigningOptions>,
            base.5,
            GetSignatureId::<Impl, OFFSET>,
            SetSignatureId::<Impl, OFFSET>,
            GetSignatureMethod::<Impl, OFFSET>,
            SetSignatureMethod::<Impl, OFFSET>,
            GetDigestMethod::<Impl, OFFSET>,
            SetDigestMethod::<Impl, OFFSET>,
            GetSignaturePartName::<Impl, OFFSET>,
            SetSignaturePartName::<Impl, OFFSET>,
            GetPolicy::<Impl, OFFSET>,
            SetPolicy::<Impl, OFFSET>,
            GetSigningTimeFormat::<Impl, OFFSET>,
            SetSigningTimeFormat::<Impl, OFFSET>,
            GetCustomObjects::<Impl, OFFSET>,
            GetCustomReferences::<Impl, OFFSET>,
            GetCertificateSet::<Impl, OFFSET>,
            GetFlags::<Impl, OFFSET>,
            SetFlags::<Impl, OFFSET>,
        )
    }
}
