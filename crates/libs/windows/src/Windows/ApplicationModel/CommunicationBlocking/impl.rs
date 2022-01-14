#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICommunicationBlockingAccessManagerStatics_Impl: Sized {
    fn IsBlockingActive(&mut self) -> ::windows::core::Result<bool>;
    fn IsBlockedNumberAsync(&mut self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowBlockNumbersUI(&mut self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowUnblockNumbersUI(&mut self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowBlockedCallsUI(&mut self) -> ::windows::core::Result<()>;
    fn ShowBlockedMessagesUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommunicationBlockingAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAccessManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAccessManagerStatics_Vtbl {
        unsafe extern "system" fn IsBlockingActive<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsBlockedNumberAsync<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowBlockNumbersUI<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowUnblockNumbersUI<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowBlockedCallsUI<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowBlockedCallsUI().into()
        }
        unsafe extern "system" fn ShowBlockedMessagesUI<Impl: ICommunicationBlockingAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowBlockedMessagesUI().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommunicationBlockingAccessManagerStatics, BASE_OFFSET>(),
            IsBlockingActive: IsBlockingActive::<Impl, IMPL_OFFSET>,
            IsBlockedNumberAsync: IsBlockedNumberAsync::<Impl, IMPL_OFFSET>,
            ShowBlockNumbersUI: ShowBlockNumbersUI::<Impl, IMPL_OFFSET>,
            ShowUnblockNumbersUI: ShowUnblockNumbersUI::<Impl, IMPL_OFFSET>,
            ShowBlockedCallsUI: ShowBlockedCallsUI::<Impl, IMPL_OFFSET>,
            ShowBlockedMessagesUI: ShowBlockedMessagesUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppManagerStatics_Impl: Sized {
    fn IsCurrentAppActiveBlockingApp(&mut self) -> ::windows::core::Result<bool>;
    fn ShowCommunicationBlockingSettingsUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICommunicationBlockingAppManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAppManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAppManagerStatics_Vtbl {
        unsafe extern "system" fn IsCurrentAppActiveBlockingApp<Impl: ICommunicationBlockingAppManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowCommunicationBlockingSettingsUI<Impl: ICommunicationBlockingAppManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowCommunicationBlockingSettingsUI().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommunicationBlockingAppManagerStatics, BASE_OFFSET>(),
            IsCurrentAppActiveBlockingApp: IsCurrentAppActiveBlockingApp::<Impl, IMPL_OFFSET>,
            ShowCommunicationBlockingSettingsUI: ShowCommunicationBlockingSettingsUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAppManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICommunicationBlockingAppManagerStatics2_Impl: Sized + ICommunicationBlockingAppManagerStatics_Impl {
    fn RequestSetAsActiveBlockingAppAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.ICommunicationBlockingAppManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAppManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAppManagerStatics2_Vtbl {
        unsafe extern "system" fn RequestSetAsActiveBlockingAppAsync<Impl: ICommunicationBlockingAppManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommunicationBlockingAppManagerStatics2, BASE_OFFSET>(),
            RequestSetAsActiveBlockingAppAsync: RequestSetAsActiveBlockingAppAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAppManagerStatics2 as ::windows::core::Interface>::IID
    }
}
