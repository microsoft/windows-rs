#[cfg(feature = "implement_exclusive")]
pub trait IClipboardContentOptionsImpl: Sized {
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAllowedInHistory(&self) -> ::windows::core::Result<bool>;
    fn SetIsAllowedInHistory(&self, value: bool) -> ::windows::core::Result<()>;
    fn RoamingFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn HistoryFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardContentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardContentOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardContentOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardContentOptionsImpl, const OFFSET: isize>() -> IClipboardContentOptionsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipboardContentOptions>, ::windows::core::GetTrustLevel, IsRoamable::<Impl, OFFSET>, SetIsRoamable::<Impl, OFFSET>, IsAllowedInHistory::<Impl, OFFSET>, SetIsAllowedInHistory::<Impl, OFFSET>, RoamingFormats::<Impl, OFFSET>, HistoryFormats::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryChangedEventArgsImpl, const OFFSET: isize>() -> IClipboardHistoryChangedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipboardHistoryChangedEventArgs>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryItemImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Content(&self) -> ::windows::core::Result<DataPackageView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardHistoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryItemImpl, const OFFSET: isize>() -> IClipboardHistoryItemVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipboardHistoryItem>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, Content::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryItemsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ClipboardHistoryItemsResultStatus>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardHistoryItemsResult {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardHistoryItemsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardHistoryItemsResultImpl, const OFFSET: isize>() -> IClipboardHistoryItemsResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipboardHistoryItemsResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Items::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardStaticsImpl: Sized {
    fn GetContent(&self) -> ::windows::core::Result<DataPackageView>;
    fn SetContent(&self, content: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn ContentChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardStaticsImpl, const OFFSET: isize>() -> IClipboardStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipboardStatics>, ::windows::core::GetTrustLevel, GetContent::<Impl, OFFSET>, SetContent::<Impl, OFFSET>, Flush::<Impl, OFFSET>, Clear::<Impl, OFFSET>, ContentChanged::<Impl, OFFSET>, RemoveContentChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardStatics2Impl: Sized {
    fn GetHistoryItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>>;
    fn ClearHistory(&self) -> ::windows::core::Result<bool>;
    fn DeleteItemFromHistory(&self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<bool>;
    fn SetHistoryItemAsContent(&self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<SetHistoryItemAsContentStatus>;
    fn IsHistoryEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsRoamingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetContentWithOptions(&self, content: &::core::option::Option<DataPackage>, options: &::core::option::Option<ClipboardContentOptions>) -> ::windows::core::Result<bool>;
    fn HistoryChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RoamingEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRoamingEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HistoryEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IClipboardStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IClipboardStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IClipboardStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipboardStatics2Impl, const OFFSET: isize>() -> IClipboardStatics2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IClipboardStatics2>,
            ::windows::core::GetTrustLevel,
            GetHistoryItemsAsync::<Impl, OFFSET>,
            ClearHistory::<Impl, OFFSET>,
            DeleteItemFromHistory::<Impl, OFFSET>,
            SetHistoryItemAsContent::<Impl, OFFSET>,
            IsHistoryEnabled::<Impl, OFFSET>,
            IsRoamingEnabled::<Impl, OFFSET>,
            SetContentWithOptions::<Impl, OFFSET>,
            HistoryChanged::<Impl, OFFSET>,
            RemoveHistoryChanged::<Impl, OFFSET>,
            RoamingEnabledChanged::<Impl, OFFSET>,
            RemoveRoamingEnabledChanged::<Impl, OFFSET>,
            HistoryEnabledChanged::<Impl, OFFSET>,
            RemoveHistoryEnabledChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageImpl: Sized {
    fn GetView(&self) -> ::windows::core::Result<DataPackageView>;
    fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySet>;
    fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation>;
    fn SetRequestedOperation(&self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn OperationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOperationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Destroyed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDestroyed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetData(&self, formatid: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetDataProvider(&self, formatid: &::windows::core::HSTRING, delayrenderer: &::core::option::Option<DataProviderHandler>) -> ::windows::core::Result<()>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetHtmlFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResourceMap(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn SetRtf(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetBitmap(&self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn SetStorageItemsReadOnly(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>) -> ::windows::core::Result<()>;
    fn SetStorageItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, readonly: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackage {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageImpl, const OFFSET: isize>() -> IDataPackageVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataPackage>,
            ::windows::core::GetTrustLevel,
            GetView::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            RequestedOperation::<Impl, OFFSET>,
            SetRequestedOperation::<Impl, OFFSET>,
            OperationCompleted::<Impl, OFFSET>,
            RemoveOperationCompleted::<Impl, OFFSET>,
            Destroyed::<Impl, OFFSET>,
            RemoveDestroyed::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            SetDataProvider::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            SetUri::<Impl, OFFSET>,
            SetHtmlFormat::<Impl, OFFSET>,
            ResourceMap::<Impl, OFFSET>,
            SetRtf::<Impl, OFFSET>,
            SetBitmap::<Impl, OFFSET>,
            SetStorageItemsReadOnly::<Impl, OFFSET>,
            SetStorageItems::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage2Impl: Sized {
    fn SetApplicationLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetWebLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackage2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage2Impl, const OFFSET: isize>() -> IDataPackage2Vtbl {
        unsafe extern "system" fn SetApplicationLink<Impl: IDataPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWebLink<Impl: IDataPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebLink(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackage2>, ::windows::core::GetTrustLevel, SetApplicationLink::<Impl, OFFSET>, SetWebLink::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage3Impl: Sized {
    fn ShareCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackage3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage3Impl, const OFFSET: isize>() -> IDataPackage3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackage3>, ::windows::core::GetTrustLevel, ShareCompleted::<Impl, OFFSET>, RemoveShareCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage4Impl: Sized {
    fn ShareCanceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackage4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackage4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackage4Impl, const OFFSET: isize>() -> IDataPackage4Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackage4>, ::windows::core::GetTrustLevel, ShareCanceled::<Impl, OFFSET>, RemoveShareCanceled::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySetImpl: Sized + IIterableImpl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetApplicationListingUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPackagePropertySet {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDataPackagePropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetImpl, const OFFSET: isize>() -> IDataPackagePropertySetVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataPackagePropertySet>,
            ::windows::core::GetTrustLevel,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            FileTypes::<Impl, OFFSET>,
            ApplicationName::<Impl, OFFSET>,
            SetApplicationName::<Impl, OFFSET>,
            ApplicationListingUri::<Impl, OFFSET>,
            SetApplicationListingUri::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet2Impl: Sized {
    fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceWebLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceApplicationLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSquare30x30Logo(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetLogoBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySet2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySet2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet2Impl, const OFFSET: isize>() -> IDataPackagePropertySet2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataPackagePropertySet2>,
            ::windows::core::GetTrustLevel,
            ContentSourceWebLink::<Impl, OFFSET>,
            SetContentSourceWebLink::<Impl, OFFSET>,
            ContentSourceApplicationLink::<Impl, OFFSET>,
            SetContentSourceApplicationLink::<Impl, OFFSET>,
            PackageFamilyName::<Impl, OFFSET>,
            SetPackageFamilyName::<Impl, OFFSET>,
            Square30x30Logo::<Impl, OFFSET>,
            SetSquare30x30Logo::<Impl, OFFSET>,
            LogoBackgroundColor::<Impl, OFFSET>,
            SetLogoBackgroundColor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySet3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySet3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet3Impl, const OFFSET: isize>() -> IDataPackagePropertySet3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySet3>, ::windows::core::GetTrustLevel, EnterpriseId::<Impl, OFFSET>, SetEnterpriseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet4Impl: Sized {
    fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentSourceUserActivityJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySet4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySet4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySet4Impl, const OFFSET: isize>() -> IDataPackagePropertySet4Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySet4>, ::windows::core::GetTrustLevel, ContentSourceUserActivityJson::<Impl, OFFSET>, SetContentSourceUserActivityJson::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetViewImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetViewImpl, const OFFSET: isize>() -> IDataPackagePropertySetViewVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySetView>, ::windows::core::GetTrustLevel, Title::<Impl, OFFSET>, Description::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>, FileTypes::<Impl, OFFSET>, ApplicationName::<Impl, OFFSET>, ApplicationListingUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView2Impl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView2Impl, const OFFSET: isize>() -> IDataPackagePropertySetView2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySetView2>, ::windows::core::GetTrustLevel, PackageFamilyName::<Impl, OFFSET>, ContentSourceWebLink::<Impl, OFFSET>, ContentSourceApplicationLink::<Impl, OFFSET>, Square30x30Logo::<Impl, OFFSET>, LogoBackgroundColor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView3Impl, const OFFSET: isize>() -> IDataPackagePropertySetView3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySetView3>, ::windows::core::GetTrustLevel, EnterpriseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView4Impl: Sized {
    fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView4Impl, const OFFSET: isize>() -> IDataPackagePropertySetView4Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySetView4>, ::windows::core::GetTrustLevel, ContentSourceUserActivityJson::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView5Impl: Sized {
    fn IsFromRoamingClipboard(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackagePropertySetView5 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackagePropertySetView5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackagePropertySetView5Impl, const OFFSET: isize>() -> IDataPackagePropertySetView5Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackagePropertySetView5>, ::windows::core::GetTrustLevel, IsFromRoamingClipboard::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageViewImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySetView>;
    fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation>;
    fn ReportOperationCompleted(&self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn AvailableFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Contains(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetDataAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomTextAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetUriAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetHtmlFormatAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetResourceMapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>>;
    fn GetRtfAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn GetStorageItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackageView {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageViewImpl, const OFFSET: isize>() -> IDataPackageViewVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataPackageView>,
            ::windows::core::GetTrustLevel,
            Properties::<Impl, OFFSET>,
            RequestedOperation::<Impl, OFFSET>,
            ReportOperationCompleted::<Impl, OFFSET>,
            AvailableFormats::<Impl, OFFSET>,
            Contains::<Impl, OFFSET>,
            GetDataAsync::<Impl, OFFSET>,
            GetTextAsync::<Impl, OFFSET>,
            GetCustomTextAsync::<Impl, OFFSET>,
            GetUriAsync::<Impl, OFFSET>,
            GetHtmlFormatAsync::<Impl, OFFSET>,
            GetResourceMapAsync::<Impl, OFFSET>,
            GetRtfAsync::<Impl, OFFSET>,
            GetBitmapAsync::<Impl, OFFSET>,
            GetStorageItemsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView2Impl: Sized {
    fn GetApplicationLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetWebLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackageView2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView2Impl, const OFFSET: isize>() -> IDataPackageView2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackageView2>, ::windows::core::GetTrustLevel, GetApplicationLinkAsync::<Impl, OFFSET>, GetWebLinkAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView3Impl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithEnterpriseIdAsync(&self, enterpriseid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn UnlockAndAssumeEnterpriseIdentity(&self) -> ::windows::core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackageView3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView3Impl, const OFFSET: isize>() -> IDataPackageView3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackageView3>, ::windows::core::GetTrustLevel, RequestAccessAsync::<Impl, OFFSET>, RequestAccessWithEnterpriseIdAsync::<Impl, OFFSET>, UnlockAndAssumeEnterpriseIdentity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView4Impl: Sized {
    fn SetAcceptedFormatId(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPackageView4 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataPackageView4";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPackageView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPackageView4Impl, const OFFSET: isize>() -> IDataPackageView4Vtbl {
        unsafe extern "system" fn SetAcceptedFormatId<Impl: IDataPackageView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceptedFormatId(&*(&formatid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPackageView4>, ::windows::core::GetTrustLevel, SetAcceptedFormatId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProviderDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProviderDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataProviderDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProviderDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProviderDeferralImpl, const OFFSET: isize>() -> IDataProviderDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDataProviderDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataProviderDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProviderRequestImpl: Sized {
    fn FormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<DataProviderDeferral>;
    fn SetData(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProviderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataProviderRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProviderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataProviderRequestImpl, const OFFSET: isize>() -> IDataProviderRequestVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataProviderRequest>, ::windows::core::GetTrustLevel, FormatId::<Impl, OFFSET>, Deadline::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, SetData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<DataPackage>;
    fn SetData(&self, value: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn FailWithDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<DataRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataRequest {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IDataRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestImpl, const OFFSET: isize>() -> IDataRequestVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataRequest>, ::windows::core::GetTrustLevel, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, Deadline::<Impl, OFFSET>, FailWithDisplayText::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDataRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestDeferralImpl, const OFFSET: isize>() -> IDataRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDataRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataRequestDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<DataRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDataRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRequestedEventArgsImpl, const OFFSET: isize>() -> IDataRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataRequestedEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerImpl: Sized {
    fn DataRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetApplicationChosen(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetApplicationChosen(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManager";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerImpl, const OFFSET: isize>() -> IDataTransferManagerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTransferManager>, ::windows::core::GetTrustLevel, DataRequested::<Impl, OFFSET>, RemoveDataRequested::<Impl, OFFSET>, TargetApplicationChosen::<Impl, OFFSET>, RemoveTargetApplicationChosen::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManager2Impl: Sized {
    fn ShareProvidersRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareProvidersRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManager2Impl, const OFFSET: isize>() -> IDataTransferManager2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTransferManager2>, ::windows::core::GetTrustLevel, ShareProvidersRequested::<Impl, OFFSET>, RemoveShareProvidersRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStaticsImpl: Sized {
    fn ShowShareUI(&self) -> ::windows::core::Result<()>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<DataTransferManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStaticsImpl, const OFFSET: isize>() -> IDataTransferManagerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTransferManagerStatics>, ::windows::core::GetTrustLevel, ShowShareUI::<Impl, OFFSET>, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStatics2Impl, const OFFSET: isize>() -> IDataTransferManagerStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTransferManagerStatics2>, ::windows::core::GetTrustLevel, IsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics3Impl: Sized {
    fn ShowShareUIWithOptions(&self, options: &::core::option::Option<ShareUIOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTransferManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTransferManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTransferManagerStatics3Impl, const OFFSET: isize>() -> IDataTransferManagerStatics3Vtbl {
        unsafe extern "system" fn ShowShareUIWithOptions<Impl: IDataTransferManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowShareUIWithOptions(&*(&options as *const <ShareUIOptions as ::windows::core::Abi>::Abi as *const <ShareUIOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTransferManagerStatics3>, ::windows::core::GetTrustLevel, ShowShareUIWithOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHtmlFormatHelperStaticsImpl: Sized {
    fn GetStaticFragment(&self, htmlformat: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateHtmlFormat(&self, htmlfragment: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHtmlFormatHelperStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHtmlFormatHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHtmlFormatHelperStaticsImpl, const OFFSET: isize>() -> IHtmlFormatHelperStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHtmlFormatHelperStatics>, ::windows::core::GetTrustLevel, GetStaticFragment::<Impl, OFFSET>, CreateHtmlFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IOperationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOperationCompletedEventArgsImpl, const OFFSET: isize>() -> IOperationCompletedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOperationCompletedEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgs2Impl: Sized {
    fn AcceptedFormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOperationCompletedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IOperationCompletedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOperationCompletedEventArgs2Impl, const OFFSET: isize>() -> IOperationCompletedEventArgs2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOperationCompletedEventArgs2>, ::windows::core::GetTrustLevel, AcceptedFormatId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareCompletedEventArgsImpl: Sized {
    fn ShareTarget(&self) -> ::windows::core::Result<ShareTargetInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShareCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareCompletedEventArgsImpl, const OFFSET: isize>() -> IShareCompletedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareCompletedEventArgs>, ::windows::core::GetTrustLevel, ShareTarget::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareProvider {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IShareProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderImpl, const OFFSET: isize>() -> IShareProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareProvider>, ::windows::core::GetTrustLevel, Title::<Impl, OFFSET>, DisplayIcon::<Impl, OFFSET>, BackgroundColor::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderFactoryImpl: Sized {
    fn Create(&self, title: &::windows::core::HSTRING, displayicon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>, backgroundcolor: &super::super::UI::Color, handler: &::core::option::Option<ShareProviderHandler>) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareProviderFactory {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProviderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IShareProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderFactoryImpl, const OFFSET: isize>() -> IShareProviderFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareProviderFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<DataPackageView>;
    fn Provider(&self) -> ::windows::core::Result<ShareProvider>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProviderOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IShareProviderOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProviderOperationImpl, const OFFSET: isize>() -> IShareProviderOperationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareProviderOperation>, ::windows::core::GetTrustLevel, Data::<Impl, OFFSET>, Provider::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProvidersRequestedEventArgsImpl: Sized {
    fn Providers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ShareProvider>>;
    fn Data(&self) -> ::windows::core::Result<DataPackageView>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareProvidersRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShareProvidersRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareProvidersRequestedEventArgsImpl, const OFFSET: isize>() -> IShareProvidersRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareProvidersRequestedEventArgs>, ::windows::core::GetTrustLevel, Providers::<Impl, OFFSET>, Data::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareTargetInfoImpl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShareProvider(&self) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareTargetInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareTargetInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IShareTargetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareTargetInfoImpl, const OFFSET: isize>() -> IShareTargetInfoVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareTargetInfo>, ::windows::core::GetTrustLevel, AppUserModelId::<Impl, OFFSET>, ShareProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareUIOptionsImpl: Sized {
    fn Theme(&self) -> ::windows::core::Result<ShareUITheme>;
    fn SetTheme(&self, value: ShareUITheme) -> ::windows::core::Result<()>;
    fn SelectionRect(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetSelectionRect(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareUIOptions {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IShareUIOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IShareUIOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareUIOptionsImpl, const OFFSET: isize>() -> IShareUIOptionsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShareUIOptions>, ::windows::core::GetTrustLevel, Theme::<Impl, OFFSET>, SetTheme::<Impl, OFFSET>, SelectionRect::<Impl, OFFSET>, SetSelectionRect::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedStorageAccessManagerStaticsImpl: Sized {
    fn AddFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RedeemTokenForFileAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn RemoveFile(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedStorageAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedStorageAccessManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedStorageAccessManagerStaticsImpl, const OFFSET: isize>() -> ISharedStorageAccessManagerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedStorageAccessManagerStatics>, ::windows::core::GetTrustLevel, AddFile::<Impl, OFFSET>, RedeemTokenForFileAsync::<Impl, OFFSET>, RemoveFile::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStaticsImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Html(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rtf(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bitmap(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageItems(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStaticsImpl, const OFFSET: isize>() -> IStandardDataFormatsStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStandardDataFormatsStatics>, ::windows::core::GetTrustLevel, Text::<Impl, OFFSET>, Uri::<Impl, OFFSET>, Html::<Impl, OFFSET>, Rtf::<Impl, OFFSET>, Bitmap::<Impl, OFFSET>, StorageItems::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics2Impl: Sized {
    fn WebLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStatics2Impl, const OFFSET: isize>() -> IStandardDataFormatsStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStandardDataFormatsStatics2>, ::windows::core::GetTrustLevel, WebLink::<Impl, OFFSET>, ApplicationLink::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics3Impl: Sized {
    fn UserActivityJsonArray(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardDataFormatsStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardDataFormatsStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardDataFormatsStatics3Impl, const OFFSET: isize>() -> IStandardDataFormatsStatics3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStandardDataFormatsStatics3>, ::windows::core::GetTrustLevel, UserActivityJsonArray::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetApplicationChosenEventArgsImpl: Sized {
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetApplicationChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetApplicationChosenEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetApplicationChosenEventArgsImpl, const OFFSET: isize>() -> ITargetApplicationChosenEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITargetApplicationChosenEventArgs>, ::windows::core::GetTrustLevel, ApplicationName::<Impl, OFFSET>)
    }
}
