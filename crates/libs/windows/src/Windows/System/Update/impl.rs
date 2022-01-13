#[cfg(feature = "implement_exclusive")]
pub trait ISystemUpdateItemImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<SystemUpdateItemState>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Revision(&mut self) -> ::windows::core::Result<u32>;
    fn DownloadProgress(&mut self) -> ::windows::core::Result<f64>;
    fn InstallProgress(&mut self) -> ::windows::core::Result<f64>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.ISystemUpdateItem";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemUpdateItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemUpdateItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemUpdateItemVtbl {
        unsafe extern "system" fn State<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateItemState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Revision<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadProgress<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallProgress<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ISystemUpdateItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemUpdateItem, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Revision: Revision::<Impl, IMPL_OFFSET>,
            DownloadProgress: DownloadProgress::<Impl, IMPL_OFFSET>,
            InstallProgress: InstallProgress::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemUpdateItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemUpdateLastErrorInfoImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<SystemUpdateManagerState>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn IsInteractive(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.ISystemUpdateLastErrorInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemUpdateLastErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemUpdateLastErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemUpdateLastErrorInfoVtbl {
        unsafe extern "system" fn State<Impl: ISystemUpdateLastErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: ISystemUpdateLastErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteractive<Impl: ISystemUpdateLastErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInteractive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemUpdateLastErrorInfo, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            IsInteractive: IsInteractive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemUpdateLastErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISystemUpdateManagerStaticsImpl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn State(&mut self) -> ::windows::core::Result<SystemUpdateManagerState>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgress(&mut self) -> ::windows::core::Result<f64>;
    fn InstallProgress(&mut self) -> ::windows::core::Result<f64>;
    fn UserActiveHoursStart(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserActiveHoursEnd(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserActiveHoursMax(&mut self) -> ::windows::core::Result<i32>;
    fn TrySetUserActiveHours(&mut self, start: &super::super::Foundation::TimeSpan, end: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<bool>;
    fn LastUpdateCheckTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastUpdateInstallTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastErrorInfo(&mut self) -> ::windows::core::Result<SystemUpdateLastErrorInfo>;
    fn GetAutomaticRebootBlockIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn BlockAutomaticRebootAsync(&mut self, lockid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UnblockAutomaticRebootAsync(&mut self, lockid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetUpdateItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>>;
    fn AttentionRequiredReason(&mut self) -> ::windows::core::Result<SystemUpdateAttentionRequiredReason>;
    fn SetFlightRing(&mut self, flightring: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetFlightRing(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartInstall(&mut self, action: SystemUpdateStartInstallAction) -> ::windows::core::Result<()>;
    fn RebootToCompleteInstall(&mut self) -> ::windows::core::Result<()>;
    fn StartCancelUpdates(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemUpdateManagerStatics {
    const NAME: &'static str = "Windows.System.Update.ISystemUpdateManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISystemUpdateManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemUpdateManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemUpdateManagerStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StateChanged<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadProgress<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallProgress<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserActiveHoursStart<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActiveHoursStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserActiveHoursEnd<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActiveHoursEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserActiveHoursMax<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActiveHoursMax() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetUserActiveHours<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetUserActiveHours(&*(&start as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&end as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUpdateCheckTime<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastUpdateCheckTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUpdateInstallTime<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastUpdateInstallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastErrorInfo<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticRebootBlockIds<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomaticRebootBlockIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockAutomaticRebootAsync<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAutomaticRebootAsync(&*(&lockid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnblockAutomaticRebootAsync<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnblockAutomaticRebootAsync(&*(&lockid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateItems<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttentionRequiredReason<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttentionRequiredReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlightRing<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flightring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlightRing(&*(&flightring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlightRing<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlightRing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartInstall<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: SystemUpdateStartInstallAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartInstall(action).into()
        }
        unsafe extern "system" fn RebootToCompleteInstall<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RebootToCompleteInstall().into()
        }
        unsafe extern "system" fn StartCancelUpdates<Impl: ISystemUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartCancelUpdates().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemUpdateManagerStatics, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            DownloadProgress: DownloadProgress::<Impl, IMPL_OFFSET>,
            InstallProgress: InstallProgress::<Impl, IMPL_OFFSET>,
            UserActiveHoursStart: UserActiveHoursStart::<Impl, IMPL_OFFSET>,
            UserActiveHoursEnd: UserActiveHoursEnd::<Impl, IMPL_OFFSET>,
            UserActiveHoursMax: UserActiveHoursMax::<Impl, IMPL_OFFSET>,
            TrySetUserActiveHours: TrySetUserActiveHours::<Impl, IMPL_OFFSET>,
            LastUpdateCheckTime: LastUpdateCheckTime::<Impl, IMPL_OFFSET>,
            LastUpdateInstallTime: LastUpdateInstallTime::<Impl, IMPL_OFFSET>,
            LastErrorInfo: LastErrorInfo::<Impl, IMPL_OFFSET>,
            GetAutomaticRebootBlockIds: GetAutomaticRebootBlockIds::<Impl, IMPL_OFFSET>,
            BlockAutomaticRebootAsync: BlockAutomaticRebootAsync::<Impl, IMPL_OFFSET>,
            UnblockAutomaticRebootAsync: UnblockAutomaticRebootAsync::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            GetUpdateItems: GetUpdateItems::<Impl, IMPL_OFFSET>,
            AttentionRequiredReason: AttentionRequiredReason::<Impl, IMPL_OFFSET>,
            SetFlightRing: SetFlightRing::<Impl, IMPL_OFFSET>,
            GetFlightRing: GetFlightRing::<Impl, IMPL_OFFSET>,
            StartInstall: StartInstall::<Impl, IMPL_OFFSET>,
            RebootToCompleteInstall: RebootToCompleteInstall::<Impl, IMPL_OFFSET>,
            StartCancelUpdates: StartCancelUpdates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemUpdateManagerStatics as ::windows::core::Interface>::IID
    }
}
