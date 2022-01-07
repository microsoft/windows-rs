pub trait IBindableIterableImpl: Sized {
    fn First(&self) -> ::windows::core::Result<IBindableIterator>;
}
impl ::windows::core::RuntimeName for IBindableIterable {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableIterable";
}
impl IBindableIterableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterableImpl, const OFFSET: isize>() -> IBindableIterableVtbl {
        unsafe extern "system" fn First<Impl: IBindableIterableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).First() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindableIterable>, ::windows::core::GetTrustLevel, First::<Impl, OFFSET>)
    }
}
pub trait IBindableIteratorImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IBindableIterator {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableIterator";
}
impl IBindableIteratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIteratorImpl, const OFFSET: isize>() -> IBindableIteratorVtbl {
        unsafe extern "system" fn Current<Impl: IBindableIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<Impl: IBindableIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IBindableIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindableIterator>, ::windows::core::GetTrustLevel, Current::<Impl, OFFSET>, HasCurrent::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
pub trait IBindableObservableVectorImpl: Sized + IBindableIterableImpl + IBindableVectorImpl {
    fn VectorChanged(&self, handler: &::core::option::Option<BindableVectorChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBindableObservableVector {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableObservableVector";
}
impl IBindableObservableVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableObservableVectorImpl, const OFFSET: isize>() -> IBindableObservableVectorVtbl {
        unsafe extern "system" fn VectorChanged<Impl: IBindableObservableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VectorChanged(&*(&handler as *const <BindableVectorChangedEventHandler as ::windows::core::Abi>::Abi as *const <BindableVectorChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<Impl: IBindableObservableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVectorChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindableObservableVector>, ::windows::core::GetTrustLevel, VectorChanged::<Impl, OFFSET>, RemoveVectorChanged::<Impl, OFFSET>)
    }
}
pub trait IBindableVectorImpl: Sized + IBindableIterableImpl {
    fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IBindableVectorView>;
    fn IndexOf(&self, value: &::core::option::Option<::windows::core::IInspectable>, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBindableVector {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableVector";
}
impl IBindableVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorImpl, const OFFSET: isize>() -> IBindableVectorVtbl {
        unsafe extern "system" fn GetAt<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexOf<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexOf(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(index, &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(index, &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<Impl: IBindableVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBindableVector>,
            ::windows::core::GetTrustLevel,
            GetAt::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            GetView::<Impl, OFFSET>,
            IndexOf::<Impl, OFFSET>,
            SetAt::<Impl, OFFSET>,
            InsertAt::<Impl, OFFSET>,
            RemoveAt::<Impl, OFFSET>,
            Append::<Impl, OFFSET>,
            RemoveAtEnd::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
        )
    }
}
pub trait IBindableVectorViewImpl: Sized + IBindableIterableImpl {
    fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &::core::option::Option<::windows::core::IInspectable>, index: &mut u32) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IBindableVectorView {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableVectorView";
}
impl IBindableVectorViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorViewImpl, const OFFSET: isize>() -> IBindableVectorViewVtbl {
        unsafe extern "system" fn GetAt<Impl: IBindableVectorViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IBindableVectorViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<Impl: IBindableVectorViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexOf(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindableVectorView>, ::windows::core::GetTrustLevel, GetAt::<Impl, OFFSET>, Size::<Impl, OFFSET>, IndexOf::<Impl, OFFSET>)
    }
}
pub trait INotifyCollectionChangedImpl: Sized {
    fn CollectionChanged(&self, handler: &::core::option::Option<NotifyCollectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INotifyCollectionChanged {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.INotifyCollectionChanged";
}
impl INotifyCollectionChangedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChangedImpl, const OFFSET: isize>() -> INotifyCollectionChangedVtbl {
        unsafe extern "system" fn CollectionChanged<Impl: INotifyCollectionChangedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionChanged(&*(&handler as *const <NotifyCollectionChangedEventHandler as ::windows::core::Abi>::Abi as *const <NotifyCollectionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCollectionChanged<Impl: INotifyCollectionChangedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCollectionChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INotifyCollectionChanged>, ::windows::core::GetTrustLevel, CollectionChanged::<Impl, OFFSET>, RemoveCollectionChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyCollectionChangedEventArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<NotifyCollectionChangedAction>;
    fn NewItems(&self) -> ::windows::core::Result<IBindableVector>;
    fn OldItems(&self) -> ::windows::core::Result<IBindableVector>;
    fn NewStartingIndex(&self) -> ::windows::core::Result<i32>;
    fn OldStartingIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INotifyCollectionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>() -> INotifyCollectionChangedEventArgsVtbl {
        unsafe extern "system" fn Action<Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotifyCollectionChangedAction) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewItems<Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldItems<Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStartingIndex<Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewStartingIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldStartingIndex<Impl: INotifyCollectionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldStartingIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INotifyCollectionChangedEventArgs>, ::windows::core::GetTrustLevel, Action::<Impl, OFFSET>, NewItems::<Impl, OFFSET>, OldItems::<Impl, OFFSET>, NewStartingIndex::<Impl, OFFSET>, OldStartingIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotifyCollectionChangedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithAllParameters(&self, action: NotifyCollectionChangedAction, newitems: &::core::option::Option<IBindableVector>, olditems: &::core::option::Option<IBindableVector>, newindex: i32, oldindex: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NotifyCollectionChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotifyCollectionChangedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.INotifyCollectionChangedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INotifyCollectionChangedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChangedEventArgsFactoryImpl, const OFFSET: isize>() -> INotifyCollectionChangedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithAllParameters<Impl: INotifyCollectionChangedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: NotifyCollectionChangedAction, newitems: ::windows::core::RawPtr, olditems: ::windows::core::RawPtr, newindex: i32, oldindex: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithAllParameters(
                action,
                &*(&newitems as *const <IBindableVector as ::windows::core::Abi>::Abi as *const <IBindableVector as ::windows::core::DefaultType>::DefaultType),
                &*(&olditems as *const <IBindableVector as ::windows::core::Abi>::Abi as *const <IBindableVector as ::windows::core::DefaultType>::DefaultType),
                newindex,
                oldindex,
                &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&innerinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INotifyCollectionChangedEventArgsFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithAllParameters::<Impl, OFFSET>)
    }
}
