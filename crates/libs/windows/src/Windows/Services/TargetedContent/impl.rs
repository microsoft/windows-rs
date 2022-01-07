#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentActionImpl: Sized {
    fn InvokeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentAction";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentActionImpl, const OFFSET: isize>() -> ITargetedContentActionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentAction>, ::windows::core::GetTrustLevel, InvokeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentAvailabilityChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentAvailabilityChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentAvailabilityChangedEventArgsImpl, const OFFSET: isize>() -> ITargetedContentAvailabilityChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentAvailabilityChangedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn HasPreviousContentExpired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentChangedEventArgsImpl, const OFFSET: isize>() -> ITargetedContentChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentChangedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>, HasPreviousContentExpired::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentCollectionImpl, const OFFSET: isize>() -> ITargetedContentCollectionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentCollection>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, ReportInteraction::<Impl, OFFSET>, ReportCustomInteraction::<Impl, OFFSET>, Path::<Impl, OFFSET>, Properties::<Impl, OFFSET>, Collections::<Impl, OFFSET>, Items::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentContainerImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Availability(&self) -> ::windows::core::Result<TargetedContentAvailability>;
    fn Content(&self) -> ::windows::core::Result<TargetedContentCollection>;
    fn SelectSingleObject(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentContainer";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentContainerImpl, const OFFSET: isize>() -> ITargetedContentContainerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentContainer>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, Availability::<Impl, OFFSET>, Content::<Impl, OFFSET>, SelectSingleObject::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentContainerStaticsImpl: Sized {
    fn GetAsync(&self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentContainerStatics {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentContainerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentContainerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentContainerStaticsImpl, const OFFSET: isize>() -> ITargetedContentContainerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentContainerStatics>, ::windows::core::GetTrustLevel, GetAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITargetedContentImageImpl: Sized + IRandomAccessStreamReferenceImpl {
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Width(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentImage";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITargetedContentImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentImageImpl, const OFFSET: isize>() -> ITargetedContentImageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentImage>, ::windows::core::GetTrustLevel, Height::<Impl, OFFSET>, Width::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentItemImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<TargetedContentItemState>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentItem";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentItemImpl, const OFFSET: isize>() -> ITargetedContentItemVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentItem>, ::windows::core::GetTrustLevel, Path::<Impl, OFFSET>, ReportInteraction::<Impl, OFFSET>, ReportCustomInteraction::<Impl, OFFSET>, State::<Impl, OFFSET>, Properties::<Impl, OFFSET>, Collections::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentItemStateImpl: Sized {
    fn ShouldDisplay(&self) -> ::windows::core::Result<bool>;
    fn AppInstallationState(&self) -> ::windows::core::Result<TargetedContentAppInstallationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentItemState";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentItemStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentItemStateImpl, const OFFSET: isize>() -> ITargetedContentItemStateVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentItemState>, ::windows::core::GetTrustLevel, ShouldDisplay::<Impl, OFFSET>, AppInstallationState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentObjectImpl: Sized {
    fn ObjectKind(&self) -> ::windows::core::Result<TargetedContentObjectKind>;
    fn Collection(&self) -> ::windows::core::Result<TargetedContentCollection>;
    fn Item(&self) -> ::windows::core::Result<TargetedContentItem>;
    fn Value(&self) -> ::windows::core::Result<TargetedContentValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentObject";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentObjectImpl, const OFFSET: isize>() -> ITargetedContentObjectVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentObject>, ::windows::core::GetTrustLevel, ObjectKind::<Impl, OFFSET>, Collection::<Impl, OFFSET>, Item::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentStateChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentStateChangedEventArgsImpl, const OFFSET: isize>() -> ITargetedContentStateChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentStateChangedEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetContentContainerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
    fn ContentChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscription";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionImpl, const OFFSET: isize>() -> ITargetedContentSubscriptionVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITargetedContentSubscription>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            GetContentContainerAsync::<Impl, OFFSET>,
            ContentChanged::<Impl, OFFSET>,
            RemoveContentChanged::<Impl, OFFSET>,
            AvailabilityChanged::<Impl, OFFSET>,
            RemoveAvailabilityChanged::<Impl, OFFSET>,
            StateChanged::<Impl, OFFSET>,
            RemoveStateChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionOptionsImpl: Sized {
    fn SubscriptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowPartialContentAvailability(&self) -> ::windows::core::Result<bool>;
    fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows::core::Result<()>;
    fn CloudQueryParameters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn LocalFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Update(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentSubscriptionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionOptionsImpl, const OFFSET: isize>() -> ITargetedContentSubscriptionOptionsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITargetedContentSubscriptionOptions>,
            ::windows::core::GetTrustLevel,
            SubscriptionId::<Impl, OFFSET>,
            AllowPartialContentAvailability::<Impl, OFFSET>,
            SetAllowPartialContentAvailability::<Impl, OFFSET>,
            CloudQueryParameters::<Impl, OFFSET>,
            LocalFilters::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionStaticsImpl: Sized {
    fn GetAsync(&self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>>;
    fn GetOptions(&self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentSubscriptionOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentSubscriptionStatics {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentSubscriptionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentSubscriptionStaticsImpl, const OFFSET: isize>() -> ITargetedContentSubscriptionStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetedContentSubscriptionStatics>, ::windows::core::GetTrustLevel, GetAsync::<Impl, OFFSET>, GetOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentValueImpl: Sized {
    fn ValueKind(&self) -> ::windows::core::Result<TargetedContentValueKind>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn String(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Number(&self) -> ::windows::core::Result<f64>;
    fn Boolean(&self) -> ::windows::core::Result<bool>;
    fn File(&self) -> ::windows::core::Result<TargetedContentFile>;
    fn ImageFile(&self) -> ::windows::core::Result<TargetedContentImage>;
    fn Action(&self) -> ::windows::core::Result<TargetedContentAction>;
    fn Strings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Uris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn Numbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>>;
    fn Booleans(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>>;
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>>;
    fn ImageFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>>;
    fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.ITargetedContentValue";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetedContentValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetedContentValueImpl, const OFFSET: isize>() -> ITargetedContentValueVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITargetedContentValue>,
            ::windows::core::GetTrustLevel,
            ValueKind::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            String::<Impl, OFFSET>,
            Uri::<Impl, OFFSET>,
            Number::<Impl, OFFSET>,
            Boolean::<Impl, OFFSET>,
            File::<Impl, OFFSET>,
            ImageFile::<Impl, OFFSET>,
            Action::<Impl, OFFSET>,
            Strings::<Impl, OFFSET>,
            Uris::<Impl, OFFSET>,
            Numbers::<Impl, OFFSET>,
            Booleans::<Impl, OFFSET>,
            Files::<Impl, OFFSET>,
            ImageFiles::<Impl, OFFSET>,
            Actions::<Impl, OFFSET>,
        )
    }
}
