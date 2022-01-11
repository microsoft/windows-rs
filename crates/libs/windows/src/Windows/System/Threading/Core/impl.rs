#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPreallocatedWorkItemImpl: Sized {
    fn RunAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPreallocatedWorkItem {
    const NAME: &'static str = "Windows.System.Threading.Core.IPreallocatedWorkItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPreallocatedWorkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreallocatedWorkItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPreallocatedWorkItemVtbl {
        unsafe extern "system" fn RunAsync<Impl: IPreallocatedWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPreallocatedWorkItem, BASE_OFFSET>(), RunAsync: RunAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPreallocatedWorkItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPreallocatedWorkItemFactoryImpl: Sized {
    fn CreateWorkItem(&self, handler: &::core::option::Option<super::WorkItemHandler>) -> ::windows::core::Result<PreallocatedWorkItem>;
    fn CreateWorkItemWithPriority(&self, handler: &::core::option::Option<super::WorkItemHandler>, priority: super::WorkItemPriority) -> ::windows::core::Result<PreallocatedWorkItem>;
    fn CreateWorkItemWithPriorityAndOptions(&self, handler: &::core::option::Option<super::WorkItemHandler>, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> ::windows::core::Result<PreallocatedWorkItem>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPreallocatedWorkItemFactory {
    const NAME: &'static str = "Windows.System.Threading.Core.IPreallocatedWorkItemFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPreallocatedWorkItemFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPreallocatedWorkItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPreallocatedWorkItemFactoryVtbl {
        unsafe extern "system" fn CreateWorkItem<Impl: IPreallocatedWorkItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWorkItem(&*(&handler as *const <super::WorkItemHandler as ::windows::core::Abi>::Abi as *const <super::WorkItemHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWorkItemWithPriority<Impl: IPreallocatedWorkItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: super::WorkItemPriority, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWorkItemWithPriority(&*(&handler as *const <super::WorkItemHandler as ::windows::core::Abi>::Abi as *const <super::WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWorkItemWithPriorityAndOptions<Impl: IPreallocatedWorkItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: super::WorkItemPriority, options: super::WorkItemOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWorkItemWithPriorityAndOptions(&*(&handler as *const <super::WorkItemHandler as ::windows::core::Abi>::Abi as *const <super::WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPreallocatedWorkItemFactory, BASE_OFFSET>(),
            CreateWorkItem: CreateWorkItem::<Impl, IMPL_OFFSET>,
            CreateWorkItemWithPriority: CreateWorkItemWithPriority::<Impl, IMPL_OFFSET>,
            CreateWorkItemWithPriorityAndOptions: CreateWorkItemWithPriorityAndOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPreallocatedWorkItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISignalNotifierImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISignalNotifier {
    const NAME: &'static str = "Windows.System.Threading.Core.ISignalNotifier";
}
#[cfg(feature = "implement_exclusive")]
impl ISignalNotifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignalNotifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignalNotifierVtbl {
        unsafe extern "system" fn Enable<Impl: ISignalNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Terminate<Impl: ISignalNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignalNotifier, BASE_OFFSET>(),
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignalNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISignalNotifierStaticsImpl: Sized {
    fn AttachToEvent(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToEventWithTimeout(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>, timeout: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToSemaphore(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToSemaphoreWithTimeout(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>, timeout: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISignalNotifierStatics {
    const NAME: &'static str = "Windows.System.Threading.Core.ISignalNotifierStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISignalNotifierStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISignalNotifierStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISignalNotifierStaticsVtbl {
        unsafe extern "system" fn AttachToEvent<Impl: ISignalNotifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToEvent(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <SignalHandler as ::windows::core::Abi>::Abi as *const <SignalHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachToEventWithTimeout<Impl: ISignalNotifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToEventWithTimeout(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <SignalHandler as ::windows::core::Abi>::Abi as *const <SignalHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&timeout as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachToSemaphore<Impl: ISignalNotifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToSemaphore(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <SignalHandler as ::windows::core::Abi>::Abi as *const <SignalHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachToSemaphoreWithTimeout<Impl: ISignalNotifierStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToSemaphoreWithTimeout(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <SignalHandler as ::windows::core::Abi>::Abi as *const <SignalHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&timeout as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISignalNotifierStatics, BASE_OFFSET>(),
            AttachToEvent: AttachToEvent::<Impl, IMPL_OFFSET>,
            AttachToEventWithTimeout: AttachToEventWithTimeout::<Impl, IMPL_OFFSET>,
            AttachToSemaphore: AttachToSemaphore::<Impl, IMPL_OFFSET>,
            AttachToSemaphoreWithTimeout: AttachToSemaphoreWithTimeout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISignalNotifierStatics as ::windows::core::Interface>::IID
    }
}
