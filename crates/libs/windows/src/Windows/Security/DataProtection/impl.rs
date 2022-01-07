#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAvailabilityStateChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.IUserDataAvailabilityStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAvailabilityStateChangedEventArgsVtbl {
    pub const fn new<Impl: IUserDataAvailabilityStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAvailabilityStateChangedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IUserDataAvailabilityStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAvailabilityStateChangedEventArgs>, base.5, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataBufferUnprotectResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<UserDataBufferUnprotectStatus>;
    fn UnprotectedBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.IUserDataBufferUnprotectResult";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataBufferUnprotectResultVtbl {
    pub const fn new<Impl: IUserDataBufferUnprotectResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataBufferUnprotectResultVtbl {
        unsafe extern "system" fn Status<Impl: IUserDataBufferUnprotectResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataBufferUnprotectStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectedBuffer<Impl: IUserDataBufferUnprotectResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprotectedBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataBufferUnprotectResult>, base.5, Status::<Impl, OFFSET>, UnprotectedBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataProtectionManagerImpl: Sized {
    fn ProtectStorageItemAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>;
    fn GetStorageItemProtectionInfoAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>;
    fn ProtectBufferAsync(&self, unprotectedbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn UnprotectBufferAsync(&self, protectedbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>;
    fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows::core::Result<bool>;
    fn DataAvailabilityStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataAvailabilityStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.IUserDataProtectionManager";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataProtectionManagerVtbl {
    pub const fn new<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataProtectionManagerVtbl {
        unsafe extern "system" fn ProtectStorageItemAsync<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtectStorageItemAsync(&*(&storageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType), availability) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageItemProtectionInfoAsync<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storageitem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStorageItemProtectionInfoAsync(&*(&storageitem as *const <super::super::Storage::IStorageItem as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectBufferAsync<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unprotectedbuffer: ::windows::core::RawPtr, availability: UserDataAvailability, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtectBufferAsync(&*(&unprotectedbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), availability) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectBufferAsync<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protectedbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprotectBufferAsync(&*(&protectedbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContinuedDataAvailabilityExpected<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContinuedDataAvailabilityExpected(availability) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataAvailabilityStateChanged<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataAvailabilityStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataAvailabilityStateChanged<Impl: IUserDataProtectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDataAvailabilityStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataProtectionManager>, base.5, ProtectStorageItemAsync::<Impl, OFFSET>, GetStorageItemProtectionInfoAsync::<Impl, OFFSET>, ProtectBufferAsync::<Impl, OFFSET>, UnprotectBufferAsync::<Impl, OFFSET>, IsContinuedDataAvailabilityExpected::<Impl, OFFSET>, DataAvailabilityStateChanged::<Impl, OFFSET>, RemoveDataAvailabilityStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataProtectionManagerStaticsImpl: Sized {
    fn TryGetDefault(&self) -> ::windows::core::Result<UserDataProtectionManager>;
    fn TryGetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserDataProtectionManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataProtectionManagerStatics {
    const NAME: &'static str = "Windows.Security.DataProtection.IUserDataProtectionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataProtectionManagerStaticsVtbl {
    pub const fn new<Impl: IUserDataProtectionManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataProtectionManagerStaticsVtbl {
        unsafe extern "system" fn TryGetDefault<Impl: IUserDataProtectionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetForUser<Impl: IUserDataProtectionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataProtectionManagerStatics>, base.5, TryGetDefault::<Impl, OFFSET>, TryGetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataStorageItemProtectionInfoImpl: Sized {
    fn Availability(&self) -> ::windows::core::Result<UserDataAvailability>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.IUserDataStorageItemProtectionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataStorageItemProtectionInfoVtbl {
    pub const fn new<Impl: IUserDataStorageItemProtectionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataStorageItemProtectionInfoVtbl {
        unsafe extern "system" fn Availability<Impl: IUserDataStorageItemProtectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataAvailability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Availability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataStorageItemProtectionInfo>, base.5, Availability::<Impl, OFFSET>)
    }
}
