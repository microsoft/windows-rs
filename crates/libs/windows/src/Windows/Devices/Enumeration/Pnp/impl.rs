#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPnpObjectImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<PnpObjectType>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Update(&mut self, updateinfo: &::core::option::Option<PnpObjectUpdate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPnpObject {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.IPnpObject";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPnpObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPnpObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPnpObjectVtbl {
        unsafe extern "system" fn Type<Impl: IPnpObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PnpObjectType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IPnpObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IPnpObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Update<Impl: IPnpObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(&*(&updateinfo as *const <PnpObjectUpdate as ::windows::core::Abi>::Abi as *const <PnpObjectUpdate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPnpObject, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPnpObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPnpObjectStaticsImpl: Sized {
    fn CreateFromIdAsync(&mut self, r#type: PnpObjectType, id: &::windows::core::HSTRING, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObject>>;
    fn FindAllAsync(&mut self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>;
    fn FindAllAsyncAqsFilter(&mut self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>;
    fn CreateWatcher(&mut self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PnpObjectWatcher>;
    fn CreateWatcherAqsFilter(&mut self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<PnpObjectWatcher>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPnpObjectStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.IPnpObjectStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPnpObjectStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPnpObjectStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPnpObjectStaticsVtbl {
        unsafe extern "system" fn CreateFromIdAsync<Impl: IPnpObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIdAsync(r#type, &*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&requestedproperties as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IPnpObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync(r#type, &*(&requestedproperties as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncAqsFilter<Impl: IPnpObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncAqsFilter(r#type, &*(&requestedproperties as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcher<Impl: IPnpObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher(r#type, &*(&requestedproperties as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherAqsFilter<Impl: IPnpObjectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherAqsFilter(r#type, &*(&requestedproperties as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPnpObjectStatics, BASE_OFFSET>(),
            CreateFromIdAsync: CreateFromIdAsync::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllAsyncAqsFilter: FindAllAsyncAqsFilter::<Impl, IMPL_OFFSET>,
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            CreateWatcherAqsFilter: CreateWatcherAqsFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPnpObjectStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPnpObjectUpdateImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<PnpObjectType>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPnpObjectUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.IPnpObjectUpdate";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPnpObjectUpdateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPnpObjectUpdateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPnpObjectUpdateVtbl {
        unsafe extern "system" fn Type<Impl: IPnpObjectUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PnpObjectType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IPnpObjectUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IPnpObjectUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPnpObjectUpdate, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPnpObjectUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPnpObjectWatcherImpl: Sized {
    fn Added(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<super::DeviceWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPnpObjectWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.IPnpObjectWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPnpObjectWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPnpObjectWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPnpObjectWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DeviceWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPnpObjectWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPnpObjectWatcher, BASE_OFFSET>(),
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPnpObjectWatcher as ::windows::core::Interface>::IID
    }
}
