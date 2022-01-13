#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentActionImpl: Sized {
    fn InvokeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentAction";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentActionVtbl {
        unsafe extern "system" fn InvokeAsync<Impl: ITargetedContentActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentAction, BASE_OFFSET>(), InvokeAsync: InvokeAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentAvailabilityChangedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentAvailabilityChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentAvailabilityChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentAvailabilityChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ITargetedContentAvailabilityChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentAvailabilityChangedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentAvailabilityChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentChangedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn HasPreviousContentExpired(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ITargetedContentChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPreviousContentExpired<Impl: ITargetedContentChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPreviousContentExpired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentChangedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            HasPreviousContentExpired: HasPreviousContentExpired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITargetedContentCollectionImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&mut self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&mut self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentCollection";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITargetedContentCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentCollectionVtbl {
        unsafe extern "system" fn Id<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInteraction<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportInteraction(interaction).into()
        }
        unsafe extern "system" fn ReportCustomInteraction<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCustomInteraction(&*(&custominteractionname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Path<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collections<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: ITargetedContentCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentCollection, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            ReportInteraction: ReportInteraction::<Impl, IMPL_OFFSET>,
            ReportCustomInteraction: ReportCustomInteraction::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Collections: Collections::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentContainerImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Availability(&mut self) -> ::windows::core::Result<TargetedContentAvailability>;
    fn Content(&mut self) -> ::windows::core::Result<TargetedContentCollection>;
    fn SelectSingleObject(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentObject>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentContainer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentContainerVtbl {
        unsafe extern "system" fn Id<Impl: ITargetedContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ITargetedContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Availability<Impl: ITargetedContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAvailability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Availability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ITargetedContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectSingleObject<Impl: ITargetedContentContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectSingleObject(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentContainer, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Availability: Availability::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            SelectSingleObject: SelectSingleObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentContainerStaticsImpl: Sized {
    fn GetAsync(&mut self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentContainerStatics {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentContainerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentContainerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentContainerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentContainerStaticsVtbl {
        unsafe extern "system" fn GetAsync<Impl: ITargetedContentContainerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsync(&*(&contentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentContainerStatics, BASE_OFFSET>(),
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentContainerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITargetedContentImageImpl: Sized + IRandomAccessStreamReferenceImpl {
    fn Height(&mut self) -> ::windows::core::Result<u32>;
    fn Width(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentImage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITargetedContentImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentImageVtbl {
        unsafe extern "system" fn Height<Impl: ITargetedContentImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: ITargetedContentImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentImage, BASE_OFFSET>(),
            Height: Height::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentImage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITargetedContentItemImpl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&mut self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&mut self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<TargetedContentItemState>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentItem";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITargetedContentItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentItemVtbl {
        unsafe extern "system" fn Path<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInteraction<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportInteraction(interaction).into()
        }
        unsafe extern "system" fn ReportCustomInteraction<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCustomInteraction(&*(&custominteractionname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn State<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collections<Impl: ITargetedContentItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentItem, BASE_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            ReportInteraction: ReportInteraction::<Impl, IMPL_OFFSET>,
            ReportCustomInteraction: ReportCustomInteraction::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Collections: Collections::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentItemStateImpl: Sized {
    fn ShouldDisplay(&mut self) -> ::windows::core::Result<bool>;
    fn AppInstallationState(&mut self) -> ::windows::core::Result<TargetedContentAppInstallationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentItemState";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentItemStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentItemStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentItemStateVtbl {
        unsafe extern "system" fn ShouldDisplay<Impl: ITargetedContentItemStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallationState<Impl: ITargetedContentItemStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAppInstallationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInstallationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentItemState, BASE_OFFSET>(),
            ShouldDisplay: ShouldDisplay::<Impl, IMPL_OFFSET>,
            AppInstallationState: AppInstallationState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentItemState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentObjectImpl: Sized {
    fn ObjectKind(&mut self) -> ::windows::core::Result<TargetedContentObjectKind>;
    fn Collection(&mut self) -> ::windows::core::Result<TargetedContentCollection>;
    fn Item(&mut self) -> ::windows::core::Result<TargetedContentItem>;
    fn Value(&mut self) -> ::windows::core::Result<TargetedContentValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentObject";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentObjectVtbl {
        unsafe extern "system" fn ObjectKind<Impl: ITargetedContentObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentObjectKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collection<Impl: ITargetedContentObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITargetedContentObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ITargetedContentObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentObject, BASE_OFFSET>(),
            ObjectKind: ObjectKind::<Impl, IMPL_OFFSET>,
            Collection: Collection::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentStateChangedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentStateChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ITargetedContentStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentStateChangedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentSubscriptionImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetContentContainerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
    fn ContentChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailabilityChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscription";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentSubscriptionVtbl {
        unsafe extern "system" fn Id<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentContainerAsync<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentContainerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContentChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AvailabilityChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailabilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailabilityChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailabilityChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentSubscription, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            GetContentContainerAsync: GetContentContainerAsync::<Impl, IMPL_OFFSET>,
            ContentChanged: ContentChanged::<Impl, IMPL_OFFSET>,
            RemoveContentChanged: RemoveContentChanged::<Impl, IMPL_OFFSET>,
            AvailabilityChanged: AvailabilityChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailabilityChanged: RemoveAvailabilityChanged::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentSubscription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITargetedContentSubscriptionOptionsImpl: Sized {
    fn SubscriptionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowPartialContentAvailability(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowPartialContentAvailability(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CloudQueryParameters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn LocalFilters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITargetedContentSubscriptionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentSubscriptionOptionsVtbl {
        unsafe extern "system" fn SubscriptionId<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowPartialContentAvailability<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPartialContentAvailability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPartialContentAvailability<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowPartialContentAvailability(value).into()
        }
        unsafe extern "system" fn CloudQueryParameters<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloudQueryParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalFilters<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentSubscriptionOptions, BASE_OFFSET>(),
            SubscriptionId: SubscriptionId::<Impl, IMPL_OFFSET>,
            AllowPartialContentAvailability: AllowPartialContentAvailability::<Impl, IMPL_OFFSET>,
            SetAllowPartialContentAvailability: SetAllowPartialContentAvailability::<Impl, IMPL_OFFSET>,
            CloudQueryParameters: CloudQueryParameters::<Impl, IMPL_OFFSET>,
            LocalFilters: LocalFilters::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentSubscriptionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITargetedContentSubscriptionStaticsImpl: Sized {
    fn GetAsync(&mut self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>>;
    fn GetOptions(&mut self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentSubscriptionOptions>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentSubscriptionStatics {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITargetedContentSubscriptionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentSubscriptionStaticsVtbl {
        unsafe extern "system" fn GetAsync<Impl: ITargetedContentSubscriptionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsync(&*(&subscriptionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Impl: ITargetedContentSubscriptionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptions(&*(&subscriptionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentSubscriptionStatics, BASE_OFFSET>(),
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentSubscriptionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITargetedContentValueImpl: Sized {
    fn ValueKind(&mut self) -> ::windows::core::Result<TargetedContentValueKind>;
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn String(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Number(&mut self) -> ::windows::core::Result<f64>;
    fn Boolean(&mut self) -> ::windows::core::Result<bool>;
    fn File(&mut self) -> ::windows::core::Result<TargetedContentFile>;
    fn ImageFile(&mut self) -> ::windows::core::Result<TargetedContentImage>;
    fn Action(&mut self) -> ::windows::core::Result<TargetedContentAction>;
    fn Strings(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Uris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn Numbers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>>;
    fn Booleans(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>>;
    fn Files(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>>;
    fn ImageFiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>>;
    fn Actions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentValue";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITargetedContentValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetedContentValueVtbl {
        unsafe extern "system" fn ValueKind<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentValueKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn String<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).String() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Number<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Number() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Boolean<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Boolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageFile<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Action<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strings<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uris<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Numbers<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Numbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Booleans<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Booleans() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Files<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageFiles<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: ITargetedContentValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetedContentValue, BASE_OFFSET>(),
            ValueKind: ValueKind::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            String: String::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Number: Number::<Impl, IMPL_OFFSET>,
            Boolean: Boolean::<Impl, IMPL_OFFSET>,
            File: File::<Impl, IMPL_OFFSET>,
            ImageFile: ImageFile::<Impl, IMPL_OFFSET>,
            Action: Action::<Impl, IMPL_OFFSET>,
            Strings: Strings::<Impl, IMPL_OFFSET>,
            Uris: Uris::<Impl, IMPL_OFFSET>,
            Numbers: Numbers::<Impl, IMPL_OFFSET>,
            Booleans: Booleans::<Impl, IMPL_OFFSET>,
            Files: Files::<Impl, IMPL_OFFSET>,
            ImageFiles: ImageFiles::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetedContentValue as ::windows::core::Interface>::IID
    }
}
