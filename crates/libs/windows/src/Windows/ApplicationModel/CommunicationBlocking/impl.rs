#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICommunicationBlockingAccessManagerStaticsImpl: Sized {
    fn IsBlockingActive(&self) -> ::windows::core::Result<bool>;
    fn IsBlockedNumberAsync(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowBlockNumbersUI(&self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowUnblockNumbersUI(&self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowBlockedCallsUI(&self) -> ::windows::core::Result<()>;
    fn ShowBlockedMessagesUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommunicationBlockingAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICommunicationBlockingAccessManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAccessManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAccessManagerStaticsVtbl {
        unsafe extern "system" fn IsBlockingActive<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlockingActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlockedNumberAsync<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlockedNumberAsync(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowBlockNumbersUI<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowBlockNumbersUI(&*(&phonenumbers as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowUnblockNumbersUI<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowUnblockNumbersUI(&*(&phonenumbers as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowBlockedCallsUI<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowBlockedCallsUI().into()
        }
        unsafe extern "system" fn ShowBlockedMessagesUI<Impl: ICommunicationBlockingAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowBlockedMessagesUI().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICommunicationBlockingAccessManagerStatics>,
            ::windows::core::GetTrustLevel,
            IsBlockingActive::<Impl, IMPL_OFFSET>,
            IsBlockedNumberAsync::<Impl, IMPL_OFFSET>,
            ShowBlockNumbersUI::<Impl, IMPL_OFFSET>,
            ShowUnblockNumbersUI::<Impl, IMPL_OFFSET>,
            ShowBlockedCallsUI::<Impl, IMPL_OFFSET>,
            ShowBlockedMessagesUI::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppManagerStaticsImpl: Sized {
    fn IsCurrentAppActiveBlockingApp(&self) -> ::windows::core::Result<bool>;
    fn ShowCommunicationBlockingSettingsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICommunicationBlockingAppManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAppManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAppManagerStaticsVtbl {
        unsafe extern "system" fn IsCurrentAppActiveBlockingApp<Impl: ICommunicationBlockingAppManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppActiveBlockingApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCommunicationBlockingSettingsUI<Impl: ICommunicationBlockingAppManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowCommunicationBlockingSettingsUI().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommunicationBlockingAppManagerStatics>, ::windows::core::GetTrustLevel, IsCurrentAppActiveBlockingApp::<Impl, IMPL_OFFSET>, ShowCommunicationBlockingSettingsUI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAppManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICommunicationBlockingAppManagerStatics2Impl: Sized + ICommunicationBlockingAppManagerStaticsImpl {
    fn RequestSetAsActiveBlockingAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommunicationBlockingAppManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAppManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAppManagerStatics2Vtbl {
        unsafe extern "system" fn RequestSetAsActiveBlockingAppAsync<Impl: ICommunicationBlockingAppManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetAsActiveBlockingAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICommunicationBlockingAppManagerStatics2>, ::windows::core::GetTrustLevel, RequestSetAsActiveBlockingAppAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAppManagerStatics2 as ::windows::core::Interface>::IID
    }
}
