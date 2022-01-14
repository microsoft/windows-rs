#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IBitmapData_Impl: Sized {
    fn CopyBytesTo(&mut self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetStride(&mut self) -> ::windows::core::Result<u32>;
    fn GetBitmapDescription(&mut self) -> ::windows::core::Result<BitmapDescription>;
    fn GetSourceBitmapDescription(&mut self) -> ::windows::core::Result<BitmapDescription>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IBitmapData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBitmapData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBitmapData_Vtbl {
        unsafe extern "system" fn CopyBytesTo<Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyBytesTo(::core::mem::transmute_copy(&sourceoffsetinbytes), ::core::mem::transmute_copy(&maxbytestocopy), ::core::mem::transmute_copy(&pvbytes), ::core::mem::transmute_copy(&numberofbytescopied)).into()
        }
        unsafe extern "system" fn GetStride<Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStride() {
                ::core::result::Result::Ok(ok__) => {
                    *pstride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapDescription<Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmapDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbitmapdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceBitmapDescription<Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceBitmapDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbitmapdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CopyBytesTo: CopyBytesTo::<Impl, IMPL_OFFSET>,
            GetStride: GetStride::<Impl, IMPL_OFFSET>,
            GetBitmapDescription: GetBitmapDescription::<Impl, IMPL_OFFSET>,
            GetSourceBitmapDescription: GetSourceBitmapDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService_Impl: Sized {
    fn AdviseVisualTreeChange(&mut self, pcallback: &::core::option::Option<IVisualTreeServiceCallback>) -> ::windows::core::Result<()>;
    fn UnadviseVisualTreeChange(&mut self, pcallback: &::core::option::Option<IVisualTreeServiceCallback>) -> ::windows::core::Result<()>;
    fn GetEnums(&mut self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()>;
    fn CreateInstance(&mut self, typename: &super::super::super::Foundation::BSTR, value: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<u64>;
    fn GetPropertyValuesChain(&mut self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()>;
    fn ClearProperty(&mut self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()>;
    fn GetCollectionCount(&mut self, instancehandle: u64) -> ::windows::core::Result<u32>;
    fn GetCollectionElements(&mut self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()>;
    fn AddChild(&mut self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()>;
    fn RemoveChild(&mut self, parent: u64, index: u32) -> ::windows::core::Result<()>;
    fn ClearChildren(&mut self, parent: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeService_Vtbl {
        unsafe extern "system" fn AdviseVisualTreeChange<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseVisualTreeChange(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn UnadviseVisualTreeChange<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseVisualTreeChange(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn GetEnums<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEnums(::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppenums)).into()
        }
        unsafe extern "system" fn CreateInstance<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&typename), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinstancehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValuesChain<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyValuesChain(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&psourcecount), ::core::mem::transmute_copy(&pppropertysources), ::core::mem::transmute_copy(&ppropertycount), ::core::mem::transmute_copy(&pppropertyvalues)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn ClearProperty<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearProperty(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn GetCollectionCount<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionCount(::core::mem::transmute_copy(&instancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcollectionsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionElements<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCollectionElements(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&pelementcount), ::core::mem::transmute_copy(&ppelementvalues)).into()
        }
        unsafe extern "system" fn AddChild<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChild(::core::mem::transmute_copy(&parent), ::core::mem::transmute_copy(&child), ::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn RemoveChild<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChild(::core::mem::transmute_copy(&parent), ::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn ClearChildren<Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearChildren(::core::mem::transmute_copy(&parent)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseVisualTreeChange: AdviseVisualTreeChange::<Impl, IMPL_OFFSET>,
            UnadviseVisualTreeChange: UnadviseVisualTreeChange::<Impl, IMPL_OFFSET>,
            GetEnums: GetEnums::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            GetPropertyValuesChain: GetPropertyValuesChain::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            ClearProperty: ClearProperty::<Impl, IMPL_OFFSET>,
            GetCollectionCount: GetCollectionCount::<Impl, IMPL_OFFSET>,
            GetCollectionElements: GetCollectionElements::<Impl, IMPL_OFFSET>,
            AddChild: AddChild::<Impl, IMPL_OFFSET>,
            RemoveChild: RemoveChild::<Impl, IMPL_OFFSET>,
            ClearChildren: ClearChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService2_Impl: Sized + IVisualTreeService_Impl {
    fn GetPropertyIndex(&mut self, object: u64, propertyname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn GetProperty(&mut self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64>;
    fn ReplaceResource(&mut self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()>;
    fn RenderTargetBitmap(&mut self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeService2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeService2_Vtbl {
        unsafe extern "system" fn GetPropertyIndex<Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyIndex(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&propertyindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceResource<Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceResource(::core::mem::transmute_copy(&resourcedictionary), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&newvalue)).into()
        }
        unsafe extern "system" fn RenderTargetBitmap<Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTargetBitmap(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&maxpixelwidth), ::core::mem::transmute_copy(&maxpixelheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbitmapdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IVisualTreeService_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPropertyIndex: GetPropertyIndex::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            ReplaceResource: ReplaceResource::<Impl, IMPL_OFFSET>,
            RenderTargetBitmap: RenderTargetBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService3_Impl: Sized + IVisualTreeService_Impl + IVisualTreeService2_Impl {
    fn ResolveResource(&mut self, resourcecontext: u64, resourcename: super::super::super::Foundation::PWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::Result<()>;
    fn GetDictionaryItem(&mut self, dictionaryhandle: u64, resourcename: super::super::super::Foundation::PWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL) -> ::windows::core::Result<u64>;
    fn AddDictionaryItem(&mut self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::Result<()>;
    fn RemoveDictionaryItem(&mut self, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeService3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeService3_Vtbl {
        unsafe extern "system" fn ResolveResource<Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: u64, resourcename: super::super::super::Foundation::PWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveResource(::core::mem::transmute_copy(&resourcecontext), ::core::mem::transmute_copy(&resourcename), ::core::mem::transmute_copy(&resourcetype), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn GetDictionaryItem<Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcename: super::super::super::Foundation::PWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute_copy(&resourcename), ::core::mem::transmute_copy(&resourceisimplicitstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *resourcehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDictionaryItem<Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute_copy(&resourcekey), ::core::mem::transmute_copy(&resourcehandle)).into()
        }
        unsafe extern "system" fn RemoveDictionaryItem<Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute_copy(&resourcekey)).into()
        }
        Self {
            base: IVisualTreeService2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ResolveResource: ResolveResource::<Impl, IMPL_OFFSET>,
            GetDictionaryItem: GetDictionaryItem::<Impl, IMPL_OFFSET>,
            AddDictionaryItem: AddDictionaryItem::<Impl, IMPL_OFFSET>,
            RemoveDictionaryItem: RemoveDictionaryItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVisualTreeServiceCallback_Impl: Sized {
    fn OnVisualTreeChange(&mut self, relation: &ParentChildRelation, element: &VisualElement, mutationtype: VisualMutationType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVisualTreeServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeServiceCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeServiceCallback_Vtbl {
        unsafe extern "system" fn OnVisualTreeChange<Impl: IVisualTreeServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVisualTreeChange(::core::mem::transmute_copy(&relation), ::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&mutationtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnVisualTreeChange: OnVisualTreeChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVisualTreeServiceCallback2_Impl: Sized + IVisualTreeServiceCallback_Impl {
    fn OnElementStateChanged(&mut self, element: u64, elementstate: VisualElementState, context: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVisualTreeServiceCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTreeServiceCallback2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTreeServiceCallback2_Vtbl {
        unsafe extern "system" fn OnElementStateChanged<Impl: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: u64, elementstate: VisualElementState, context: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnElementStateChanged(::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&elementstate), ::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: IVisualTreeServiceCallback_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnElementStateChanged: OnElementStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXamlDiagnostics_Impl: Sized {
    fn GetDispatcher(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetUiLayer(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetApplication(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetIInspectableFromHandle(&mut self, instancehandle: u64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetHandleFromIInspectable(&mut self, pinstance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<u64>;
    fn HitTest(&mut self, rect: &super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::Result<()>;
    fn RegisterInstance(&mut self, pinstance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<u64>;
    fn GetInitializationData(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXamlDiagnostics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDiagnostics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlDiagnostics_Vtbl {
        unsafe extern "system" fn GetDispatcher<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiLayer<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiLayer() {
                ::core::result::Result::Ok(ok__) => {
                    *pplayer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIInspectableFromHandle<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIInspectableFromHandle(::core::mem::transmute_copy(&instancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHandleFromIInspectable<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHandleFromIInspectable(::core::mem::transmute(&pinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTest<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HitTest(::core::mem::transmute_copy(&rect), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppinstancehandles)).into()
        }
        unsafe extern "system" fn RegisterInstance<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterInstance(::core::mem::transmute(&pinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinstancehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitializationData<Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInitializationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pinitializationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDispatcher: GetDispatcher::<Impl, IMPL_OFFSET>,
            GetUiLayer: GetUiLayer::<Impl, IMPL_OFFSET>,
            GetApplication: GetApplication::<Impl, IMPL_OFFSET>,
            GetIInspectableFromHandle: GetIInspectableFromHandle::<Impl, IMPL_OFFSET>,
            GetHandleFromIInspectable: GetHandleFromIInspectable::<Impl, IMPL_OFFSET>,
            HitTest: HitTest::<Impl, IMPL_OFFSET>,
            RegisterInstance: RegisterInstance::<Impl, IMPL_OFFSET>,
            GetInitializationData: GetInitializationData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlDiagnostics as ::windows::core::Interface>::IID
    }
}
