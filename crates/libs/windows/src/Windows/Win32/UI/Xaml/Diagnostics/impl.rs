pub trait IBitmapDataImpl: Sized {
    fn CopyBytesTo();
    fn GetStride();
    fn GetBitmapDescription();
    fn GetSourceBitmapDescription();
}
impl ::windows::core::RuntimeName for IBitmapData {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IBitmapData";
}
impl IBitmapDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapDataImpl, const OFFSET: isize>() -> IBitmapDataVtbl {
        unsafe extern "system" fn CopyBytesTo<Impl: IBitmapDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyBytesTo(sourceoffsetinbytes, maxbytestocopy, ::core::mem::transmute_copy(&pvbytes), ::core::mem::transmute_copy(&numberofbytescopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStride<Impl: IBitmapDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStride(::core::mem::transmute_copy(&pstride)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapDescription<Impl: IBitmapDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmapDescription(::core::mem::transmute_copy(&pbitmapdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceBitmapDescription<Impl: IBitmapDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceBitmapDescription(::core::mem::transmute_copy(&pbitmapdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBitmapData>, ::windows::core::GetTrustLevel, CopyBytesTo::<Impl, OFFSET>, GetStride::<Impl, OFFSET>, GetBitmapDescription::<Impl, OFFSET>, GetSourceBitmapDescription::<Impl, OFFSET>)
    }
}
pub trait IVisualTreeServiceImpl: Sized {
    fn AdviseVisualTreeChange();
    fn UnadviseVisualTreeChange();
    fn GetEnums();
    fn CreateInstance();
    fn GetPropertyValuesChain();
    fn SetProperty();
    fn ClearProperty();
    fn GetCollectionCount();
    fn GetCollectionElements();
    fn AddChild();
    fn RemoveChild();
    fn ClearChildren();
}
impl ::windows::core::RuntimeName for IVisualTreeService {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeService";
}
impl IVisualTreeServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeServiceImpl, const OFFSET: isize>() -> IVisualTreeServiceVtbl {
        unsafe extern "system" fn AdviseVisualTreeChange<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseVisualTreeChange(&*(&pcallback as *const <IVisualTreeServiceCallback as ::windows::core::Abi>::Abi as *const <IVisualTreeServiceCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseVisualTreeChange<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseVisualTreeChange(&*(&pcallback as *const <IVisualTreeServiceCallback as ::windows::core::Abi>::Abi as *const <IVisualTreeServiceCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnums<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnums(::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppenums)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&typename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinstancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValuesChain<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValuesChain(instancehandle, ::core::mem::transmute_copy(&psourcecount), ::core::mem::transmute_copy(&pppropertysources), ::core::mem::transmute_copy(&ppropertycount), ::core::mem::transmute_copy(&pppropertyvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(instancehandle, value, propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearProperty<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearProperty(instancehandle, propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionCount<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionCount(instancehandle, ::core::mem::transmute_copy(&pcollectionsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionElements<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionElements(instancehandle, startindex, pelementcount, ::core::mem::transmute_copy(&ppelementvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddChild<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddChild(parent, child, index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChild<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveChild(parent, index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChildren<Impl: IVisualTreeServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearChildren(parent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVisualTreeService>,
            ::windows::core::GetTrustLevel,
            AdviseVisualTreeChange::<Impl, OFFSET>,
            UnadviseVisualTreeChange::<Impl, OFFSET>,
            GetEnums::<Impl, OFFSET>,
            CreateInstance::<Impl, OFFSET>,
            GetPropertyValuesChain::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            ClearProperty::<Impl, OFFSET>,
            GetCollectionCount::<Impl, OFFSET>,
            GetCollectionElements::<Impl, OFFSET>,
            AddChild::<Impl, OFFSET>,
            RemoveChild::<Impl, OFFSET>,
            ClearChildren::<Impl, OFFSET>,
        )
    }
}
pub trait IVisualTreeService2Impl: Sized + IVisualTreeServiceImpl {
    fn GetPropertyIndex();
    fn GetProperty();
    fn ReplaceResource();
    fn RenderTargetBitmap();
}
impl ::windows::core::RuntimeName for IVisualTreeService2 {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeService2";
}
impl IVisualTreeService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeService2Impl, const OFFSET: isize>() -> IVisualTreeService2Vtbl {
        unsafe extern "system" fn GetPropertyIndex<Impl: IVisualTreeService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyIndex(object, &*(&propertyname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropertyindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IVisualTreeService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(object, propertyindex, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceResource<Impl: IVisualTreeService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceResource(resourcedictionary, key, newvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTargetBitmap<Impl: IVisualTreeService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTargetBitmap(handle, options, maxpixelwidth, maxpixelheight, ::core::mem::transmute_copy(&ppbitmapdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVisualTreeService2>, ::windows::core::GetTrustLevel, GetPropertyIndex::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, ReplaceResource::<Impl, OFFSET>, RenderTargetBitmap::<Impl, OFFSET>)
    }
}
pub trait IVisualTreeService3Impl: Sized + IVisualTreeService2Impl + IVisualTreeServiceImpl {
    fn ResolveResource();
    fn GetDictionaryItem();
    fn AddDictionaryItem();
    fn RemoveDictionaryItem();
}
impl ::windows::core::RuntimeName for IVisualTreeService3 {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeService3";
}
impl IVisualTreeService3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeService3Impl, const OFFSET: isize>() -> IVisualTreeService3Vtbl {
        unsafe extern "system" fn ResolveResource<Impl: IVisualTreeService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: u64, resourcename: super::super::super::Foundation::PWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveResource(resourcecontext, &*(&resourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), resourcetype, propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDictionaryItem<Impl: IVisualTreeService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcename: super::super::super::Foundation::PWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryItem(dictionaryhandle, &*(&resourcename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&resourceisimplicitstyle as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resourcehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDictionaryItem<Impl: IVisualTreeService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDictionaryItem(dictionaryhandle, resourcekey, resourcehandle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDictionaryItem<Impl: IVisualTreeService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveDictionaryItem(dictionaryhandle, resourcekey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVisualTreeService3>, ::windows::core::GetTrustLevel, ResolveResource::<Impl, OFFSET>, GetDictionaryItem::<Impl, OFFSET>, AddDictionaryItem::<Impl, OFFSET>, RemoveDictionaryItem::<Impl, OFFSET>)
    }
}
pub trait IVisualTreeServiceCallbackImpl: Sized {
    fn OnVisualTreeChange();
}
impl ::windows::core::RuntimeName for IVisualTreeServiceCallback {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeServiceCallback";
}
impl IVisualTreeServiceCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeServiceCallbackImpl, const OFFSET: isize>() -> IVisualTreeServiceCallbackVtbl {
        unsafe extern "system" fn OnVisualTreeChange<Impl: IVisualTreeServiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnVisualTreeChange(&*(&relation as *const <ParentChildRelation as ::windows::core::Abi>::Abi as *const <ParentChildRelation as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <VisualElement as ::windows::core::Abi>::Abi as *const <VisualElement as ::windows::core::DefaultType>::DefaultType), mutationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVisualTreeServiceCallback>, ::windows::core::GetTrustLevel, OnVisualTreeChange::<Impl, OFFSET>)
    }
}
pub trait IVisualTreeServiceCallback2Impl: Sized + IVisualTreeServiceCallbackImpl {
    fn OnElementStateChanged();
}
impl ::windows::core::RuntimeName for IVisualTreeServiceCallback2 {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IVisualTreeServiceCallback2";
}
impl IVisualTreeServiceCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeServiceCallback2Impl, const OFFSET: isize>() -> IVisualTreeServiceCallback2Vtbl {
        unsafe extern "system" fn OnElementStateChanged<Impl: IVisualTreeServiceCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: u64, elementstate: VisualElementState, context: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnElementStateChanged(element, elementstate, &*(&context as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVisualTreeServiceCallback2>, ::windows::core::GetTrustLevel, OnElementStateChanged::<Impl, OFFSET>)
    }
}
pub trait IXamlDiagnosticsImpl: Sized {
    fn GetDispatcher();
    fn GetUiLayer();
    fn GetApplication();
    fn GetIInspectableFromHandle();
    fn GetHandleFromIInspectable();
    fn HitTest();
    fn RegisterInstance();
    fn GetInitializationData();
}
impl ::windows::core::RuntimeName for IXamlDiagnostics {
    const NAME: &'static str = "Windows.Win32.UI.Xaml.Diagnostics.IXamlDiagnostics";
}
impl IXamlDiagnosticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDiagnosticsImpl, const OFFSET: isize>() -> IXamlDiagnosticsVtbl {
        unsafe extern "system" fn GetDispatcher<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDispatcher(::core::mem::transmute_copy(&ppdispatcher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiLayer<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiLayer(::core::mem::transmute_copy(&pplayer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplication(::core::mem::transmute_copy(&ppapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIInspectableFromHandle<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIInspectableFromHandle(instancehandle, ::core::mem::transmute_copy(&ppinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHandleFromIInspectable<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHandleFromIInspectable(&*(&pinstance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTest<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HitTest(&*(&rect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppinstancehandles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterInstance<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterInstance(&*(&pinstance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pinstancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitializationData<Impl: IXamlDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInitializationData(::core::mem::transmute_copy(&pinitializationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXamlDiagnostics>,
            ::windows::core::GetTrustLevel,
            GetDispatcher::<Impl, OFFSET>,
            GetUiLayer::<Impl, OFFSET>,
            GetApplication::<Impl, OFFSET>,
            GetIInspectableFromHandle::<Impl, OFFSET>,
            GetHandleFromIInspectable::<Impl, OFFSET>,
            HitTest::<Impl, OFFSET>,
            RegisterInstance::<Impl, OFFSET>,
            GetInitializationData::<Impl, OFFSET>,
        )
    }
}
