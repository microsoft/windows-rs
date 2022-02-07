pub trait IBindableIterable_Impl: Sized {
    fn First(&self) -> ::windows::core::Result<IBindableIterator>;
}
impl ::windows::core::RuntimeName for IBindableIterable {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableIterable";
}
impl IBindableIterable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterable_Impl, const OFFSET: isize>() -> IBindableIterable_Vtbl {
        unsafe extern "system" fn First<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).First() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBindableIterable, OFFSET>(), First: First::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindableIterable as ::windows::core::Interface>::IID
    }
}
pub trait IBindableIterator_Impl: Sized {
    fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IBindableIterator {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableIterator";
}
impl IBindableIterator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterator_Impl, const OFFSET: isize>() -> IBindableIterator_Vtbl {
        unsafe extern "system" fn Current<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl, Impl: IBindableIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBindableIterator, OFFSET>(),
            Current: Current::<Identity, Impl, OFFSET>,
            HasCurrent: HasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindableIterator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBindableObservableVector_Impl: Sized + IBindableIterable_Impl + IBindableVector_Impl {
    fn VectorChanged(&self, handler: &::core::option::Option<BindableVectorChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBindableObservableVector {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableObservableVector";
}
#[cfg(feature = "Foundation")]
impl IBindableObservableVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableObservableVector_Impl, const OFFSET: isize>() -> IBindableObservableVector_Vtbl {
        unsafe extern "system" fn VectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IBindableObservableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VectorChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<Identity: ::windows::core::IUnknownImpl, Impl: IBindableObservableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveVectorChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBindableObservableVector, OFFSET>(),
            VectorChanged: VectorChanged::<Identity, Impl, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindableObservableVector as ::windows::core::Interface>::IID
    }
}
pub trait IBindableVector_Impl: Sized + IBindableIterable_Impl {
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
impl IBindableVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>() -> IBindableVector_Vtbl {
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IndexOf(::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Append(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBindableVector, OFFSET>(),
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindableVector as ::windows::core::Interface>::IID
    }
}
pub trait IBindableVectorView_Impl: Sized + IBindableIterable_Impl {
    fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &::core::option::Option<::windows::core::IInspectable>, index: &mut u32) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IBindableVectorView {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.IBindableVectorView";
}
impl IBindableVectorView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorView_Impl, const OFFSET: isize>() -> IBindableVectorView_Vtbl {
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<Identity: ::windows::core::IUnknownImpl, Impl: IBindableVectorView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IndexOf(::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBindableVectorView, OFFSET>(),
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindableVectorView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INotifyCollectionChanged_Impl: Sized {
    fn CollectionChanged(&self, handler: &::core::option::Option<NotifyCollectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for INotifyCollectionChanged {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.INotifyCollectionChanged";
}
#[cfg(feature = "Foundation")]
impl INotifyCollectionChanged_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChanged_Impl, const OFFSET: isize>() -> INotifyCollectionChanged_Vtbl {
        unsafe extern "system" fn CollectionChanged<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCollectionChanged<Identity: ::windows::core::IUnknownImpl, Impl: INotifyCollectionChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCollectionChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotifyCollectionChanged, OFFSET>(),
            CollectionChanged: CollectionChanged::<Identity, Impl, OFFSET>,
            RemoveCollectionChanged: RemoveCollectionChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotifyCollectionChanged as ::windows::core::Interface>::IID
    }
}
