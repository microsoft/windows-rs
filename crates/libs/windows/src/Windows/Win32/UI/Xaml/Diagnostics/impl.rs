#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IBitmapData_Impl: Sized {
    fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetStride(&self) -> ::windows::core::Result<u32>;
    fn GetBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription>;
    fn GetSourceBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for IBitmapData {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IBitmapData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBitmapData_Impl, const OFFSET: isize>() -> IBitmapData_Vtbl {
        unsafe extern "system" fn CopyBytesTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyBytesTo(::core::mem::transmute_copy(&sourceoffsetinbytes), ::core::mem::transmute_copy(&maxbytestocopy), ::core::mem::transmute_copy(&pvbytes), ::core::mem::transmute_copy(&numberofbytescopied)).into()
        }
        unsafe extern "system" fn GetStride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstride: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstride, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBitmapDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitmapdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceBitmapDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBitmapData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceBitmapDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitmapdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CopyBytesTo: CopyBytesTo::<Identity, Impl, OFFSET>,
            GetStride: GetStride::<Identity, Impl, OFFSET>,
            GetBitmapDescription: GetBitmapDescription::<Identity, Impl, OFFSET>,
            GetSourceBitmapDescription: GetSourceBitmapDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBitmapData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService_Impl: Sized {
    fn AdviseVisualTreeChange(&self, pcallback: &::core::option::Option<IVisualTreeServiceCallback>) -> ::windows::core::Result<()>;
    fn UnadviseVisualTreeChange(&self, pcallback: &::core::option::Option<IVisualTreeServiceCallback>) -> ::windows::core::Result<()>;
    fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()>;
    fn CreateInstance(&self, typename: &super::super::super::Foundation::BSTR, value: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<u64>;
    fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()>;
    fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()>;
    fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()>;
    fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32>;
    fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()>;
    fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()>;
    fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()>;
    fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IVisualTreeService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>() -> IVisualTreeService_Vtbl {
        unsafe extern "system" fn AdviseVisualTreeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseVisualTreeChange(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn UnadviseVisualTreeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseVisualTreeChange(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn GetEnums<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEnums(::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppenums)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&typename), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstancehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValuesChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyValuesChain(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&psourcecount), ::core::mem::transmute_copy(&pppropertysources), ::core::mem::transmute_copy(&ppropertycount), ::core::mem::transmute_copy(&pppropertyvalues)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn ClearProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearProperty(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn GetCollectionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollectionCount(::core::mem::transmute_copy(&instancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcollectionsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionElements<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCollectionElements(::core::mem::transmute_copy(&instancehandle), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&pelementcount), ::core::mem::transmute_copy(&ppelementvalues)).into()
        }
        unsafe extern "system" fn AddChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddChild(::core::mem::transmute_copy(&parent), ::core::mem::transmute_copy(&child), ::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn RemoveChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveChild(::core::mem::transmute_copy(&parent), ::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn ClearChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearChildren(::core::mem::transmute_copy(&parent)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AdviseVisualTreeChange: AdviseVisualTreeChange::<Identity, Impl, OFFSET>,
            UnadviseVisualTreeChange: UnadviseVisualTreeChange::<Identity, Impl, OFFSET>,
            GetEnums: GetEnums::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetPropertyValuesChain: GetPropertyValuesChain::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            ClearProperty: ClearProperty::<Identity, Impl, OFFSET>,
            GetCollectionCount: GetCollectionCount::<Identity, Impl, OFFSET>,
            GetCollectionElements: GetCollectionElements::<Identity, Impl, OFFSET>,
            AddChild: AddChild::<Identity, Impl, OFFSET>,
            RemoveChild: RemoveChild::<Identity, Impl, OFFSET>,
            ClearChildren: ClearChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService2_Impl: Sized + IVisualTreeService_Impl {
    fn GetPropertyIndex(&self, object: u64, propertyname: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64>;
    fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()>;
    fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IVisualTreeService2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService2_Impl, const OFFSET: isize>() -> IVisualTreeService2_Vtbl {
        unsafe extern "system" fn GetPropertyIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyname: ::windows::core::PCWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyIndex(::core::mem::transmute_copy(&object), ::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&propertyindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplaceResource(::core::mem::transmute_copy(&resourcedictionary), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&newvalue)).into()
        }
        unsafe extern "system" fn RenderTargetBitmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RenderTargetBitmap(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&maxpixelwidth), ::core::mem::transmute_copy(&maxpixelheight)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbitmapdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IVisualTreeService_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyIndex: GetPropertyIndex::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            ReplaceResource: ReplaceResource::<Identity, Impl, OFFSET>,
            RenderTargetBitmap: RenderTargetBitmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService2 as ::windows::core::Interface>::IID || iid == &<IVisualTreeService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IVisualTreeService3_Impl: Sized + IVisualTreeService_Impl + IVisualTreeService2_Impl {
    fn ResolveResource(&self, resourcecontext: u64, resourcename: &::windows::core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::Result<()>;
    fn GetDictionaryItem(&self, dictionaryhandle: u64, resourcename: &::windows::core::PCWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL) -> ::windows::core::Result<u64>;
    fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::Result<()>;
    fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IVisualTreeService3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IVisualTreeService3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService3_Impl, const OFFSET: isize>() -> IVisualTreeService3_Vtbl {
        unsafe extern "system" fn ResolveResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: u64, resourcename: ::windows::core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveResource(::core::mem::transmute_copy(&resourcecontext), ::core::mem::transmute(&resourcename), ::core::mem::transmute_copy(&resourcetype), ::core::mem::transmute_copy(&propertyindex)).into()
        }
        unsafe extern "system" fn GetDictionaryItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcename: ::windows::core::PCWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute(&resourcename), ::core::mem::transmute_copy(&resourceisimplicitstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDictionaryItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute_copy(&resourcekey), ::core::mem::transmute_copy(&resourcehandle)).into()
        }
        unsafe extern "system" fn RemoveDictionaryItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDictionaryItem(::core::mem::transmute_copy(&dictionaryhandle), ::core::mem::transmute_copy(&resourcekey)).into()
        }
        Self {
            base__: IVisualTreeService2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ResolveResource: ResolveResource::<Identity, Impl, OFFSET>,
            GetDictionaryItem: GetDictionaryItem::<Identity, Impl, OFFSET>,
            AddDictionaryItem: AddDictionaryItem::<Identity, Impl, OFFSET>,
            RemoveDictionaryItem: RemoveDictionaryItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeService3 as ::windows::core::Interface>::IID || iid == &<IVisualTreeService as ::windows::core::Interface>::IID || iid == &<IVisualTreeService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVisualTreeServiceCallback_Impl: Sized {
    fn OnVisualTreeChange(&self, relation: &ParentChildRelation, element: &VisualElement, mutationtype: VisualMutationType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IVisualTreeServiceCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IVisualTreeServiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeServiceCallback_Impl, const OFFSET: isize>() -> IVisualTreeServiceCallback_Vtbl {
        unsafe extern "system" fn OnVisualTreeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeServiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnVisualTreeChange(::core::mem::transmute(&relation), ::core::mem::transmute(&element), ::core::mem::transmute_copy(&mutationtype)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnVisualTreeChange: OnVisualTreeChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVisualTreeServiceCallback2_Impl: Sized + IVisualTreeServiceCallback_Impl {
    fn OnElementStateChanged(&self, element: u64, elementstate: VisualElementState, context: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IVisualTreeServiceCallback2 {}
#[cfg(feature = "Win32_Foundation")]
impl IVisualTreeServiceCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>() -> IVisualTreeServiceCallback2_Vtbl {
        unsafe extern "system" fn OnElementStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: u64, elementstate: VisualElementState, context: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnElementStateChanged(::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&elementstate), ::core::mem::transmute(&context)).into()
        }
        Self { base__: IVisualTreeServiceCallback_Vtbl::new::<Identity, Impl, OFFSET>(), OnElementStateChanged: OnElementStateChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback2 as ::windows::core::Interface>::IID || iid == &<IVisualTreeServiceCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXamlDiagnostics_Impl: Sized {
    fn GetDispatcher(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetUiLayer(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetApplication(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetIInspectableFromHandle(&self, instancehandle: u64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetHandleFromIInspectable(&self, pinstance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<u64>;
    fn HitTest(&self, rect: &super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::Result<()>;
    fn RegisterInstance(&self, pinstance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<u64>;
    fn GetInitializationData(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXamlDiagnostics {}
#[cfg(feature = "Win32_Foundation")]
impl IXamlDiagnostics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>() -> IXamlDiagnostics_Vtbl {
        unsafe extern "system" fn GetDispatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiLayer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUiLayer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplayer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIInspectableFromHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIInspectableFromHandle(::core::mem::transmute_copy(&instancehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinstance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHandleFromIInspectable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHandleFromIInspectable(::core::mem::transmute(&pinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HitTest(::core::mem::transmute(&rect), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ppinstancehandles)).into()
        }
        unsafe extern "system" fn RegisterInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterInstance(::core::mem::transmute(&pinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstancehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitializationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInitializationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinitializationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDispatcher: GetDispatcher::<Identity, Impl, OFFSET>,
            GetUiLayer: GetUiLayer::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            GetIInspectableFromHandle: GetIInspectableFromHandle::<Identity, Impl, OFFSET>,
            GetHandleFromIInspectable: GetHandleFromIInspectable::<Identity, Impl, OFFSET>,
            HitTest: HitTest::<Identity, Impl, OFFSET>,
            RegisterInstance: RegisterInstance::<Identity, Impl, OFFSET>,
            GetInitializationData: GetInitializationData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlDiagnostics as ::windows::core::Interface>::IID
    }
}
