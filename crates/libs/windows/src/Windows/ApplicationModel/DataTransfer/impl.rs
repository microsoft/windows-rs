#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IClipboardContentOptionsImpl: Sized {
    fn IsRoamable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAllowedInHistory(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAllowedInHistory(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RoamingFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn HistoryFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClipboardContentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardContentOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IClipboardContentOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardContentOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardContentOptionsVtbl {
        unsafe extern "system" fn IsRoamable<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoamable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoamable<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRoamable(value).into()
        }
        unsafe extern "system" fn IsAllowedInHistory<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAllowedInHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAllowedInHistory<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAllowedInHistory(value).into()
        }
        unsafe extern "system" fn RoamingFormats<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HistoryFormats<Impl: IClipboardContentOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoryFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardContentOptions, BASE_OFFSET>(),
            IsRoamable: IsRoamable::<Impl, IMPL_OFFSET>,
            SetIsRoamable: SetIsRoamable::<Impl, IMPL_OFFSET>,
            IsAllowedInHistory: IsAllowedInHistory::<Impl, IMPL_OFFSET>,
            SetIsAllowedInHistory: SetIsAllowedInHistory::<Impl, IMPL_OFFSET>,
            RoamingFormats: RoamingFormats::<Impl, IMPL_OFFSET>,
            HistoryFormats: HistoryFormats::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardContentOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardHistoryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardHistoryChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardHistoryChangedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardHistoryChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardHistoryChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClipboardHistoryItemImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Content(&mut self) -> ::windows::core::Result<DataPackageView>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClipboardHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClipboardHistoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardHistoryItemVtbl {
        unsafe extern "system" fn Id<Impl: IClipboardHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: IClipboardHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Content<Impl: IClipboardHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardHistoryItem, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardHistoryItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IClipboardHistoryItemsResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<ClipboardHistoryItemsResultStatus>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClipboardHistoryItemsResult {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IClipboardHistoryItemsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryItemsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardHistoryItemsResultVtbl {
        unsafe extern "system" fn Status<Impl: IClipboardHistoryItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ClipboardHistoryItemsResultStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: IClipboardHistoryItemsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardHistoryItemsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardHistoryItemsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClipboardStaticsImpl: Sized {
    fn GetContent(&mut self) -> ::windows::core::Result<DataPackageView>;
    fn SetContent(&mut self, content: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ContentChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClipboardStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClipboardStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardStaticsVtbl {
        unsafe extern "system" fn GetContent<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&content as *const <DataPackage as ::windows::core::Abi>::Abi as *const <DataPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Flush<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn Clear<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ContentChanged<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentChanged<Impl: IClipboardStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContentChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardStatics, BASE_OFFSET>(),
            GetContent: GetContent::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ContentChanged: ContentChanged::<Impl, IMPL_OFFSET>,
            RemoveContentChanged: RemoveContentChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClipboardStatics2Impl: Sized {
    fn GetHistoryItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>>;
    fn ClearHistory(&mut self) -> ::windows::core::Result<bool>;
    fn DeleteItemFromHistory(&mut self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<bool>;
    fn SetHistoryItemAsContent(&mut self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<SetHistoryItemAsContentStatus>;
    fn IsHistoryEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsRoamingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetContentWithOptions(&mut self, content: &::core::option::Option<DataPackage>, options: &::core::option::Option<ClipboardContentOptions>) -> ::windows::core::Result<bool>;
    fn HistoryChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RoamingEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRoamingEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HistoryEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClipboardStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClipboardStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipboardStatics2Vtbl {
        unsafe extern "system" fn GetHistoryItemsAsync<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistoryItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearHistory<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItemFromHistory<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteItemFromHistory(&*(&item as *const <ClipboardHistoryItem as ::windows::core::Abi>::Abi as *const <ClipboardHistoryItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHistoryItemAsContent<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut SetHistoryItemAsContentStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHistoryItemAsContent(&*(&item as *const <ClipboardHistoryItem as ::windows::core::Abi>::Abi as *const <ClipboardHistoryItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHistoryEnabled<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHistoryEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoamingEnabled<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoamingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentWithOptions<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContentWithOptions(&*(&content as *const <DataPackage as ::windows::core::Abi>::Abi as *const <DataPackage as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <ClipboardContentOptions as ::windows::core::Abi>::Abi as *const <ClipboardContentOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HistoryChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoryChanged(&*(&handler as *const <super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHistoryChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHistoryChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RoamingEnabledChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingEnabledChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRoamingEnabledChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRoamingEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HistoryEnabledChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HistoryEnabledChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHistoryEnabledChanged<Impl: IClipboardStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHistoryEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClipboardStatics2, BASE_OFFSET>(),
            GetHistoryItemsAsync: GetHistoryItemsAsync::<Impl, IMPL_OFFSET>,
            ClearHistory: ClearHistory::<Impl, IMPL_OFFSET>,
            DeleteItemFromHistory: DeleteItemFromHistory::<Impl, IMPL_OFFSET>,
            SetHistoryItemAsContent: SetHistoryItemAsContent::<Impl, IMPL_OFFSET>,
            IsHistoryEnabled: IsHistoryEnabled::<Impl, IMPL_OFFSET>,
            IsRoamingEnabled: IsRoamingEnabled::<Impl, IMPL_OFFSET>,
            SetContentWithOptions: SetContentWithOptions::<Impl, IMPL_OFFSET>,
            HistoryChanged: HistoryChanged::<Impl, IMPL_OFFSET>,
            RemoveHistoryChanged: RemoveHistoryChanged::<Impl, IMPL_OFFSET>,
            RoamingEnabledChanged: RoamingEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveRoamingEnabledChanged: RemoveRoamingEnabledChanged::<Impl, IMPL_OFFSET>,
            HistoryEnabledChanged: HistoryEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveHistoryEnabledChanged: RemoveHistoryEnabledChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipboardStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDataPackageImpl: Sized {
    fn GetView(&mut self) -> ::windows::core::Result<DataPackageView>;
    fn Properties(&mut self) -> ::windows::core::Result<DataPackagePropertySet>;
    fn RequestedOperation(&mut self) -> ::windows::core::Result<DataPackageOperation>;
    fn SetRequestedOperation(&mut self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn OperationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOperationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Destroyed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDestroyed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetData(&mut self, formatid: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetDataProvider(&mut self, formatid: &::windows::core::HSTRING, delayrenderer: &::core::option::Option<DataProviderHandler>) -> ::windows::core::Result<()>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetHtmlFormat(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResourceMap(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn SetRtf(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetBitmap(&mut self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn SetStorageItemsReadOnly(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>) -> ::windows::core::Result<()>;
    fn SetStorageItems(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, readonly: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDataPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackageVtbl {
        unsafe extern "system" fn GetView<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestedOperation<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedOperation<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedOperation(value).into()
        }
        unsafe extern "system" fn OperationCompleted<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOperationCompleted<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOperationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Destroyed<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroyed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDestroyed<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDestroyed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetData<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDataProvider<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, delayrenderer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataProvider(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&delayrenderer as *const <DataProviderHandler as ::windows::core::Abi>::Abi as *const <DataProviderHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetText<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetUri<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetHtmlFormat<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHtmlFormat(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResourceMap<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceMap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRtf<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRtf(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBitmap<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmap(&*(&value as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStorageItemsReadOnly<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageItemsReadOnly(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStorageItems<Impl: IDataPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, readonly: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageItems(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem> as ::windows::core::DefaultType>::DefaultType), readonly).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackage, BASE_OFFSET>(),
            GetView: GetView::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            RequestedOperation: RequestedOperation::<Impl, IMPL_OFFSET>,
            SetRequestedOperation: SetRequestedOperation::<Impl, IMPL_OFFSET>,
            OperationCompleted: OperationCompleted::<Impl, IMPL_OFFSET>,
            RemoveOperationCompleted: RemoveOperationCompleted::<Impl, IMPL_OFFSET>,
            Destroyed: Destroyed::<Impl, IMPL_OFFSET>,
            RemoveDestroyed: RemoveDestroyed::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            SetDataProvider: SetDataProvider::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            SetHtmlFormat: SetHtmlFormat::<Impl, IMPL_OFFSET>,
            ResourceMap: ResourceMap::<Impl, IMPL_OFFSET>,
            SetRtf: SetRtf::<Impl, IMPL_OFFSET>,
            SetBitmap: SetBitmap::<Impl, IMPL_OFFSET>,
            SetStorageItemsReadOnly: SetStorageItemsReadOnly::<Impl, IMPL_OFFSET>,
            SetStorageItems: SetStorageItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPackage2Impl: Sized {
    fn SetApplicationLink(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetWebLink(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackage2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPackage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackage2Vtbl {
        unsafe extern "system" fn SetApplicationLink<Impl: IDataPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWebLink<Impl: IDataPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackage2, BASE_OFFSET>(),
            SetApplicationLink: SetApplicationLink::<Impl, IMPL_OFFSET>,
            SetWebLink: SetWebLink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPackage3Impl: Sized {
    fn ShareCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackage3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPackage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackage3Vtbl {
        unsafe extern "system" fn ShareCompleted<Impl: IDataPackage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShareCompleted<Impl: IDataPackage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShareCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackage3, BASE_OFFSET>(),
            ShareCompleted: ShareCompleted::<Impl, IMPL_OFFSET>,
            RemoveShareCompleted: RemoveShareCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPackage4Impl: Sized {
    fn ShareCanceled(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCanceled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackage4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPackage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackage4Vtbl {
        unsafe extern "system" fn ShareCanceled<Impl: IDataPackage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareCanceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShareCanceled<Impl: IDataPackage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShareCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackage4, BASE_OFFSET>(),
            ShareCanceled: ShareCanceled::<Impl, IMPL_OFFSET>,
            RemoveShareCanceled: RemoveShareCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySetImpl: Sized + IIterableImpl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn FileTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ApplicationName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ApplicationListingUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetApplicationListingUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackagePropertySet {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDataPackagePropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetVtbl {
        unsafe extern "system" fn Title<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileTypes<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationName<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ApplicationListingUri<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationListingUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationListingUri<Impl: IDataPackagePropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationListingUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySet, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            FileTypes: FileTypes::<Impl, IMPL_OFFSET>,
            ApplicationName: ApplicationName::<Impl, IMPL_OFFSET>,
            SetApplicationName: SetApplicationName::<Impl, IMPL_OFFSET>,
            ApplicationListingUri: ApplicationListingUri::<Impl, IMPL_OFFSET>,
            SetApplicationListingUri: SetApplicationListingUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySet2Impl: Sized {
    fn ContentSourceWebLink(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceWebLink(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentSourceApplicationLink(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceApplicationLink(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Square30x30Logo(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSquare30x30Logo(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn LogoBackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetLogoBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackagePropertySet2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl IDataPackagePropertySet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySet2Vtbl {
        unsafe extern "system" fn ContentSourceWebLink<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceWebLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentSourceWebLink<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentSourceWebLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentSourceApplicationLink<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceApplicationLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentSourceApplicationLink<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentSourceApplicationLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPackageFamilyName<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square30x30Logo<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square30x30Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSquare30x30Logo<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare30x30Logo(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogoBackgroundColor<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogoBackgroundColor<Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogoBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySet2, BASE_OFFSET>(),
            ContentSourceWebLink: ContentSourceWebLink::<Impl, IMPL_OFFSET>,
            SetContentSourceWebLink: SetContentSourceWebLink::<Impl, IMPL_OFFSET>,
            ContentSourceApplicationLink: ContentSourceApplicationLink::<Impl, IMPL_OFFSET>,
            SetContentSourceApplicationLink: SetContentSourceApplicationLink::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
            SetPackageFamilyName: SetPackageFamilyName::<Impl, IMPL_OFFSET>,
            Square30x30Logo: Square30x30Logo::<Impl, IMPL_OFFSET>,
            SetSquare30x30Logo: SetSquare30x30Logo::<Impl, IMPL_OFFSET>,
            LogoBackgroundColor: LogoBackgroundColor::<Impl, IMPL_OFFSET>,
            SetLogoBackgroundColor: SetLogoBackgroundColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySet2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet3Impl: Sized {
    fn EnterpriseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySet3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySet3Vtbl {
        unsafe extern "system" fn EnterpriseId<Impl: IDataPackagePropertySet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnterpriseId<Impl: IDataPackagePropertySet3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySet3, BASE_OFFSET>(),
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
            SetEnterpriseId: SetEnterpriseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySet3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet4Impl: Sized {
    fn ContentSourceUserActivityJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentSourceUserActivityJson(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySet4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySet4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySet4Vtbl {
        unsafe extern "system" fn ContentSourceUserActivityJson<Impl: IDataPackagePropertySet4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceUserActivityJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentSourceUserActivityJson<Impl: IDataPackagePropertySet4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentSourceUserActivityJson(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySet4, BASE_OFFSET>(),
            ContentSourceUserActivityJson: ContentSourceUserActivityJson::<Impl, IMPL_OFFSET>,
            SetContentSourceUserActivityJson: SetContentSourceUserActivityJson::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySet4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySetViewImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn FileTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ApplicationName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationListingUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDataPackagePropertySetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetViewVtbl {
        unsafe extern "system" fn Title<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileTypes<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationName<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationListingUri<Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationListingUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySetView, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            FileTypes: FileTypes::<Impl, IMPL_OFFSET>,
            ApplicationName: ApplicationName::<Impl, IMPL_OFFSET>,
            ApplicationListingUri: ApplicationListingUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySetView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySetView2Impl: Sized {
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentSourceWebLink(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ContentSourceApplicationLink(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Square30x30Logo(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn LogoBackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl IDataPackagePropertySetView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetView2Vtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentSourceWebLink<Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceWebLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentSourceApplicationLink<Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceApplicationLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Square30x30Logo<Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square30x30Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogoBackgroundColor<Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySetView2, BASE_OFFSET>(),
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
            ContentSourceWebLink: ContentSourceWebLink::<Impl, IMPL_OFFSET>,
            ContentSourceApplicationLink: ContentSourceApplicationLink::<Impl, IMPL_OFFSET>,
            Square30x30Logo: Square30x30Logo::<Impl, IMPL_OFFSET>,
            LogoBackgroundColor: LogoBackgroundColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySetView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView3Impl: Sized {
    fn EnterpriseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetView3Vtbl {
        unsafe extern "system" fn EnterpriseId<Impl: IDataPackagePropertySetView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySetView3, BASE_OFFSET>(),
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySetView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView4Impl: Sized {
    fn ContentSourceUserActivityJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetView4Vtbl {
        unsafe extern "system" fn ContentSourceUserActivityJson<Impl: IDataPackagePropertySetView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentSourceUserActivityJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySetView4, BASE_OFFSET>(),
            ContentSourceUserActivityJson: ContentSourceUserActivityJson::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySetView4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView5Impl: Sized {
    fn IsFromRoamingClipboard(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView5 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackagePropertySetView5Vtbl {
        unsafe extern "system" fn IsFromRoamingClipboard<Impl: IDataPackagePropertySetView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFromRoamingClipboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackagePropertySetView5, BASE_OFFSET>(),
            IsFromRoamingClipboard: IsFromRoamingClipboard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackagePropertySetView5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDataPackageViewImpl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<DataPackagePropertySetView>;
    fn RequestedOperation(&mut self) -> ::windows::core::Result<DataPackageOperation>;
    fn ReportOperationCompleted(&mut self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn AvailableFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Contains(&mut self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetDataAsync(&mut self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetTextAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomTextAsync(&mut self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetUriAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetHtmlFormatAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetResourceMapAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>>;
    fn GetRtfAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetBitmapAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn GetStorageItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDataPackageViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackageViewVtbl {
        unsafe extern "system" fn Properties<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestedOperation<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportOperationCompleted<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportOperationCompleted(value).into()
        }
        unsafe extern "system" fn AvailableFormats<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contains<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contains(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataAsync(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomTextAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomTextAsync(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUriAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUriAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHtmlFormatAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHtmlFormatAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceMapAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceMapAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRtfAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRtfAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmapAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageItemsAsync<Impl: IDataPackageViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackageView, BASE_OFFSET>(),
            Properties: Properties::<Impl, IMPL_OFFSET>,
            RequestedOperation: RequestedOperation::<Impl, IMPL_OFFSET>,
            ReportOperationCompleted: ReportOperationCompleted::<Impl, IMPL_OFFSET>,
            AvailableFormats: AvailableFormats::<Impl, IMPL_OFFSET>,
            Contains: Contains::<Impl, IMPL_OFFSET>,
            GetDataAsync: GetDataAsync::<Impl, IMPL_OFFSET>,
            GetTextAsync: GetTextAsync::<Impl, IMPL_OFFSET>,
            GetCustomTextAsync: GetCustomTextAsync::<Impl, IMPL_OFFSET>,
            GetUriAsync: GetUriAsync::<Impl, IMPL_OFFSET>,
            GetHtmlFormatAsync: GetHtmlFormatAsync::<Impl, IMPL_OFFSET>,
            GetResourceMapAsync: GetResourceMapAsync::<Impl, IMPL_OFFSET>,
            GetRtfAsync: GetRtfAsync::<Impl, IMPL_OFFSET>,
            GetBitmapAsync: GetBitmapAsync::<Impl, IMPL_OFFSET>,
            GetStorageItemsAsync: GetStorageItemsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackageView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPackageView2Impl: Sized {
    fn GetApplicationLinkAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetWebLinkAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackageView2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPackageView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackageView2Vtbl {
        unsafe extern "system" fn GetApplicationLinkAsync<Impl: IDataPackageView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationLinkAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebLinkAsync<Impl: IDataPackageView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWebLinkAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackageView2, BASE_OFFSET>(),
            GetApplicationLinkAsync: GetApplicationLinkAsync::<Impl, IMPL_OFFSET>,
            GetWebLinkAsync: GetWebLinkAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackageView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData", feature = "implement_exclusive"))]
pub trait IDataPackageView3Impl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithEnterpriseIdAsync(&mut self, enterpriseid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn UnlockAndAssumeEnterpriseIdentity(&mut self) -> ::windows::core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>;
}
#[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackageView3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView3";
}
#[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData", feature = "implement_exclusive"))]
impl IDataPackageView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackageView3Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IDataPackageView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessWithEnterpriseIdAsync<Impl: IDataPackageView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enterpriseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessWithEnterpriseIdAsync(&*(&enterpriseid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockAndAssumeEnterpriseIdentity<Impl: IDataPackageView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockAndAssumeEnterpriseIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackageView3, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RequestAccessWithEnterpriseIdAsync: RequestAccessWithEnterpriseIdAsync::<Impl, IMPL_OFFSET>,
            UnlockAndAssumeEnterpriseIdentity: UnlockAndAssumeEnterpriseIdentity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackageView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView4Impl: Sized {
    fn SetAcceptedFormatId(&mut self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackageView4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPackageView4Vtbl {
        unsafe extern "system" fn SetAcceptedFormatId<Impl: IDataPackageView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceptedFormatId(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPackageView4, BASE_OFFSET>(),
            SetAcceptedFormatId: SetAcceptedFormatId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPackageView4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProviderDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProviderDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataProviderDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProviderDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProviderDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataProviderDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDataProviderDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataProviderDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataProviderDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataProviderRequestImpl: Sized {
    fn FormatId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<DataProviderDeferral>;
    fn SetData(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataProviderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataProviderRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataProviderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProviderRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataProviderRequestVtbl {
        unsafe extern "system" fn FormatId<Impl: IDataProviderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: IDataProviderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IDataProviderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IDataProviderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataProviderRequest, BASE_OFFSET>(),
            FormatId: FormatId::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataProviderRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataRequestImpl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<DataPackage>;
    fn SetData(&mut self, value: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn FailWithDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<DataRequestDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRequestVtbl {
        unsafe extern "system" fn Data<Impl: IDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <DataPackage as ::windows::core::Abi>::Abi as *const <DataPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Deadline<Impl: IDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FailWithDisplayText<Impl: IDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FailWithDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataRequest, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            FailWithDisplayText: FailWithDisplayText::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDataRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDataRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataRequestDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<DataRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDataRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IDataRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataRequestedEventArgs, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataTransferManagerImpl: Sized {
    fn DataRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetApplicationChosen(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetApplicationChosen(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataTransferManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataTransferManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTransferManagerVtbl {
        unsafe extern "system" fn DataRequested<Impl: IDataTransferManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataRequested<Impl: IDataTransferManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetApplicationChosen<Impl: IDataTransferManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetApplicationChosen(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetApplicationChosen<Impl: IDataTransferManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetApplicationChosen(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTransferManager, BASE_OFFSET>(),
            DataRequested: DataRequested::<Impl, IMPL_OFFSET>,
            RemoveDataRequested: RemoveDataRequested::<Impl, IMPL_OFFSET>,
            TargetApplicationChosen: TargetApplicationChosen::<Impl, IMPL_OFFSET>,
            RemoveTargetApplicationChosen: RemoveTargetApplicationChosen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTransferManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataTransferManager2Impl: Sized {
    fn ShareProvidersRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareProvidersRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataTransferManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataTransferManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTransferManager2Vtbl {
        unsafe extern "system" fn ShareProvidersRequested<Impl: IDataTransferManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareProvidersRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShareProvidersRequested<Impl: IDataTransferManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShareProvidersRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTransferManager2, BASE_OFFSET>(),
            ShareProvidersRequested: ShareProvidersRequested::<Impl, IMPL_OFFSET>,
            RemoveShareProvidersRequested: RemoveShareProvidersRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTransferManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStaticsImpl: Sized {
    fn ShowShareUI(&mut self) -> ::windows::core::Result<()>;
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<DataTransferManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTransferManagerStaticsVtbl {
        unsafe extern "system" fn ShowShareUI<Impl: IDataTransferManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowShareUI().into()
        }
        unsafe extern "system" fn GetForCurrentView<Impl: IDataTransferManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTransferManagerStatics, BASE_OFFSET>(),
            ShowShareUI: ShowShareUI::<Impl, IMPL_OFFSET>,
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTransferManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics2Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTransferManagerStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IDataTransferManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTransferManagerStatics2, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTransferManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics3Impl: Sized {
    fn ShowShareUIWithOptions(&mut self, options: &::core::option::Option<ShareUIOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTransferManagerStatics3Vtbl {
        unsafe extern "system" fn ShowShareUIWithOptions<Impl: IDataTransferManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowShareUIWithOptions(&*(&options as *const <ShareUIOptions as ::windows::core::Abi>::Abi as *const <ShareUIOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTransferManagerStatics3, BASE_OFFSET>(),
            ShowShareUIWithOptions: ShowShareUIWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTransferManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHtmlFormatHelperStaticsImpl: Sized {
    fn GetStaticFragment(&mut self, htmlformat: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateHtmlFormat(&mut self, htmlfragment: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHtmlFormatHelperStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHtmlFormatHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHtmlFormatHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHtmlFormatHelperStaticsVtbl {
        unsafe extern "system" fn GetStaticFragment<Impl: IHtmlFormatHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlformat: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStaticFragment(&*(&htmlformat as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHtmlFormat<Impl: IHtmlFormatHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlfragment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHtmlFormat(&*(&htmlfragment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHtmlFormatHelperStatics, BASE_OFFSET>(),
            GetStaticFragment: GetStaticFragment::<Impl, IMPL_OFFSET>,
            CreateHtmlFormat: CreateHtmlFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHtmlFormatHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgsImpl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOperationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOperationCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOperationCompletedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IOperationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IOperationCompletedEventArgs, BASE_OFFSET>(), Operation: Operation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOperationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgs2Impl: Sized {
    fn AcceptedFormatId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOperationCompletedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IOperationCompletedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOperationCompletedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOperationCompletedEventArgs2Vtbl {
        unsafe extern "system" fn AcceptedFormatId<Impl: IOperationCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptedFormatId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOperationCompletedEventArgs2, BASE_OFFSET>(),
            AcceptedFormatId: AcceptedFormatId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOperationCompletedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareCompletedEventArgsImpl: Sized {
    fn ShareTarget(&mut self) -> ::windows::core::Result<ShareTargetInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShareCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareCompletedEventArgsVtbl {
        unsafe extern "system" fn ShareTarget<Impl: IShareCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IShareCompletedEventArgs, BASE_OFFSET>(), ShareTarget: ShareTarget::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
pub trait IShareProviderImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayIcon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShareProvider {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl IShareProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareProviderVtbl {
        unsafe extern "system" fn Title<Impl: IShareProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayIcon<Impl: IShareProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundColor<Impl: IShareProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IShareProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IShareProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareProvider, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            DisplayIcon: DisplayIcon::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
pub trait IShareProviderFactoryImpl: Sized {
    fn Create(&mut self, title: &::windows::core::HSTRING, displayicon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>, backgroundcolor: &super::super::UI::Color, handler: &::core::option::Option<ShareProviderHandler>) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShareProviderFactory {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProviderFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl IShareProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareProviderFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IShareProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayicon: ::windows::core::RawPtr, backgroundcolor: super::super::UI::Color, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayicon as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
                &*(&backgroundcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <ShareProviderHandler as ::windows::core::Abi>::Abi as *const <ShareProviderHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IShareProviderFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareProviderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderOperationImpl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<DataPackageView>;
    fn Provider(&mut self) -> ::windows::core::Result<ShareProvider>;
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProviderOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IShareProviderOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareProviderOperationVtbl {
        unsafe extern "system" fn Data<Impl: IShareProviderOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Provider<Impl: IShareProviderOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Provider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IShareProviderOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareProviderOperation, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            Provider: Provider::<Impl, IMPL_OFFSET>,
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareProviderOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IShareProvidersRequestedEventArgsImpl: Sized {
    fn Providers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ShareProvider>>;
    fn Data(&mut self) -> ::windows::core::Result<DataPackageView>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShareProvidersRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IShareProvidersRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProvidersRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareProvidersRequestedEventArgsVtbl {
        unsafe extern "system" fn Providers<Impl: IShareProvidersRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Providers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IShareProvidersRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IShareProvidersRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareProvidersRequestedEventArgs, BASE_OFFSET>(),
            Providers: Providers::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareProvidersRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareTargetInfoImpl: Sized {
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShareProvider(&mut self) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareTargetInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareTargetInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IShareTargetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareTargetInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareTargetInfoVtbl {
        unsafe extern "system" fn AppUserModelId<Impl: IShareTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareProvider<Impl: IShareTargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareTargetInfo, BASE_OFFSET>(),
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            ShareProvider: ShareProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareTargetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShareUIOptionsImpl: Sized {
    fn Theme(&mut self) -> ::windows::core::Result<ShareUITheme>;
    fn SetTheme(&mut self, value: ShareUITheme) -> ::windows::core::Result<()>;
    fn SelectionRect(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetSelectionRect(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShareUIOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareUIOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShareUIOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareUIOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareUIOptionsVtbl {
        unsafe extern "system" fn Theme<Impl: IShareUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ShareUITheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Theme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTheme<Impl: IShareUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ShareUITheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTheme(value).into()
        }
        unsafe extern "system" fn SelectionRect<Impl: IShareUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionRect<Impl: IShareUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionRect(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareUIOptions, BASE_OFFSET>(),
            Theme: Theme::<Impl, IMPL_OFFSET>,
            SetTheme: SetTheme::<Impl, IMPL_OFFSET>,
            SelectionRect: SelectionRect::<Impl, IMPL_OFFSET>,
            SetSelectionRect: SetSelectionRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareUIOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ISharedStorageAccessManagerStaticsImpl: Sized {
    fn AddFile(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RedeemTokenForFileAsync(&mut self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn RemoveFile(&mut self, token: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISharedStorageAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ISharedStorageAccessManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedStorageAccessManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedStorageAccessManagerStaticsVtbl {
        unsafe extern "system" fn AddFile<Impl: ISharedStorageAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFile(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedeemTokenForFileAsync<Impl: ISharedStorageAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedeemTokenForFileAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Impl: ISharedStorageAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFile(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedStorageAccessManagerStatics, BASE_OFFSET>(),
            AddFile: AddFile::<Impl, IMPL_OFFSET>,
            RedeemTokenForFileAsync: RedeemTokenForFileAsync::<Impl, IMPL_OFFSET>,
            RemoveFile: RemoveFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedStorageAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStaticsImpl: Sized {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Html(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rtf(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bitmap(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageItems(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardDataFormatsStaticsVtbl {
        unsafe extern "system" fn Text<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Html<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Html() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rtf<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rtf() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bitmap<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageItems<Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardDataFormatsStatics, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Html: Html::<Impl, IMPL_OFFSET>,
            Rtf: Rtf::<Impl, IMPL_OFFSET>,
            Bitmap: Bitmap::<Impl, IMPL_OFFSET>,
            StorageItems: StorageItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardDataFormatsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics2Impl: Sized {
    fn WebLink(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationLink(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardDataFormatsStatics2Vtbl {
        unsafe extern "system" fn WebLink<Impl: IStandardDataFormatsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationLink<Impl: IStandardDataFormatsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardDataFormatsStatics2, BASE_OFFSET>(),
            WebLink: WebLink::<Impl, IMPL_OFFSET>,
            ApplicationLink: ApplicationLink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardDataFormatsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics3Impl: Sized {
    fn UserActivityJsonArray(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardDataFormatsStatics3Vtbl {
        unsafe extern "system" fn UserActivityJsonArray<Impl: IStandardDataFormatsStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActivityJsonArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardDataFormatsStatics3, BASE_OFFSET>(),
            UserActivityJsonArray: UserActivityJsonArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardDataFormatsStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetApplicationChosenEventArgsImpl: Sized {
    fn ApplicationName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetApplicationChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetApplicationChosenEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetApplicationChosenEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetApplicationChosenEventArgsVtbl {
        unsafe extern "system" fn ApplicationName<Impl: ITargetApplicationChosenEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetApplicationChosenEventArgs, BASE_OFFSET>(),
            ApplicationName: ApplicationName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetApplicationChosenEventArgs as ::windows::core::Interface>::IID
    }
}
