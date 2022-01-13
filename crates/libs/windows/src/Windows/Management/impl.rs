#[cfg(feature = "implement_exclusive")]
pub trait IMdmAlertImpl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Format(&mut self) -> ::windows::core::Result<MdmAlertDataType>;
    fn SetFormat(&mut self, value: MdmAlertDataType) -> ::windows::core::Result<()>;
    fn Mark(&mut self) -> ::windows::core::Result<MdmAlertMark>;
    fn SetMark(&mut self, value: MdmAlertMark) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<u32>;
    fn Target(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTarget(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMdmAlert {
    const NAME: &'static str = "Windows.Management.IMdmAlert";
}
#[cfg(feature = "implement_exclusive")]
impl IMdmAlertVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMdmAlertImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMdmAlertVtbl {
        unsafe extern "system" fn Data<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Format<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MdmAlertDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn Mark<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertMark) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMark<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MdmAlertMark) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMark(value).into()
        }
        unsafe extern "system" fn Source<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Target<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IMdmAlertImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMdmAlert, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            Mark: Mark::<Impl, IMPL_OFFSET>,
            SetMark: SetMark::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Target: Target::<Impl, IMPL_OFFSET>,
            SetTarget: SetTarget::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMdmAlert as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMdmSessionImpl: Sized {
    fn Alerts(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<MdmAlert>>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<MdmSessionState>;
    fn AttachAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn StartWithAlertsAsync(&mut self, alerts: &::core::option::Option<super::Foundation::Collections::IIterable<MdmAlert>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMdmSession {
    const NAME: &'static str = "Windows.Management.IMdmSession";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMdmSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMdmSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMdmSessionVtbl {
        unsafe extern "system" fn Alerts<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alerts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MdmSessionState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttachAsync<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn StartAsync<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartWithAlertsAsync<Impl: IMdmSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alerts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartWithAlertsAsync(&*(&alerts as *const <super::Foundation::Collections::IIterable<MdmAlert> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<MdmAlert> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMdmSession, BASE_OFFSET>(),
            Alerts: Alerts::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            AttachAsync: AttachAsync::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StartWithAlertsAsync: StartWithAlertsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMdmSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMdmSessionManagerStaticsImpl: Sized {
    fn SessionIds(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn TryCreateSession(&mut self) -> ::windows::core::Result<MdmSession>;
    fn DeleteSessionById(&mut self, sessionid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetSessionById(&mut self, sessionid: &::windows::core::HSTRING) -> ::windows::core::Result<MdmSession>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMdmSessionManagerStatics {
    const NAME: &'static str = "Windows.Management.IMdmSessionManagerStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMdmSessionManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMdmSessionManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMdmSessionManagerStaticsVtbl {
        unsafe extern "system" fn SessionIds<Impl: IMdmSessionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateSession<Impl: IMdmSessionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSessionById<Impl: IMdmSessionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSessionById(&*(&sessionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSessionById<Impl: IMdmSessionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionById(&*(&sessionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMdmSessionManagerStatics, BASE_OFFSET>(),
            SessionIds: SessionIds::<Impl, IMPL_OFFSET>,
            TryCreateSession: TryCreateSession::<Impl, IMPL_OFFSET>,
            DeleteSessionById: DeleteSessionById::<Impl, IMPL_OFFSET>,
            GetSessionById: GetSessionById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMdmSessionManagerStatics as ::windows::core::Interface>::IID
    }
}
