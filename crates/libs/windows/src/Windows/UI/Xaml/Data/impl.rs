#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait ICollectionView_Impl: Sized + super::super::super::Foundation::Collections::IIterable_Impl<::windows::core::IInspectable> + super::super::super::Foundation::Collections::IObservableVector_Impl<::windows::core::IInspectable> + super::super::super::Foundation::Collections::IVector_Impl<::windows::core::IInspectable> {
    fn CurrentItem(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CurrentPosition(&mut self) -> ::windows::core::Result<i32>;
    fn IsCurrentAfterLast(&mut self) -> ::windows::core::Result<bool>;
    fn IsCurrentBeforeFirst(&mut self) -> ::windows::core::Result<bool>;
    fn CollectionGroups(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
    fn HasMoreItems(&mut self) -> ::windows::core::Result<bool>;
    fn CurrentChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentChanging(&mut self, handler: &::core::option::Option<CurrentChangingEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanging(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MoveCurrentTo(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPosition(&mut self, index: i32) -> ::windows::core::Result<bool>;
    fn MoveCurrentToFirst(&mut self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToLast(&mut self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToNext(&mut self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPrevious(&mut self) -> ::windows::core::Result<bool>;
    fn LoadMoreItemsAsync(&mut self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for ICollectionView {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionView";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ICollectionView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>() -> ICollectionView_Vtbl {
        unsafe extern "system" fn CurrentItem<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPosition<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAfterLast<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCurrentAfterLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentBeforeFirst<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCurrentBeforeFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionGroups<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCurrentChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn CurrentChanging<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentChanging(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanging<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCurrentChanging(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn MoveCurrentTo<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentTo(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPosition<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentToPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToFirst<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentToFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToLast<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentToLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToNext<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentToNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPrevious<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveCurrentToPrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMoreItemsAsync<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionView, OFFSET>(),
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            CurrentPosition: CurrentPosition::<Identity, Impl, OFFSET>,
            IsCurrentAfterLast: IsCurrentAfterLast::<Identity, Impl, OFFSET>,
            IsCurrentBeforeFirst: IsCurrentBeforeFirst::<Identity, Impl, OFFSET>,
            CollectionGroups: CollectionGroups::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
            CurrentChanged: CurrentChanged::<Identity, Impl, OFFSET>,
            RemoveCurrentChanged: RemoveCurrentChanged::<Identity, Impl, OFFSET>,
            CurrentChanging: CurrentChanging::<Identity, Impl, OFFSET>,
            RemoveCurrentChanging: RemoveCurrentChanging::<Identity, Impl, OFFSET>,
            MoveCurrentTo: MoveCurrentTo::<Identity, Impl, OFFSET>,
            MoveCurrentToPosition: MoveCurrentToPosition::<Identity, Impl, OFFSET>,
            MoveCurrentToFirst: MoveCurrentToFirst::<Identity, Impl, OFFSET>,
            MoveCurrentToLast: MoveCurrentToLast::<Identity, Impl, OFFSET>,
            MoveCurrentToNext: MoveCurrentToNext::<Identity, Impl, OFFSET>,
            MoveCurrentToPrevious: MoveCurrentToPrevious::<Identity, Impl, OFFSET>,
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICollectionView as ::windows::core::Interface>::IID
    }
}
pub trait ICollectionViewFactory_Impl: Sized {
    fn CreateView(&mut self) -> ::windows::core::Result<ICollectionView>;
}
impl ::windows::core::RuntimeName for ICollectionViewFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewFactory";
}
impl ICollectionViewFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewFactory_Impl, const OFFSET: isize>() -> ICollectionViewFactory_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionViewFactory, OFFSET>(), CreateView: CreateView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICollectionViewFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICollectionViewGroup_Impl: Sized {
    fn Group(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GroupItems(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ICollectionViewGroup {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICollectionViewGroup";
}
#[cfg(feature = "Foundation_Collections")]
impl ICollectionViewGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>() -> ICollectionViewGroup_Vtbl {
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupItems<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GroupItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionViewGroup, OFFSET>(),
            Group: Group::<Identity, Impl, OFFSET>,
            GroupItems: GroupItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICollectionViewGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait ICustomProperty_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValue(&mut self, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetIndexedValue(&mut self, target: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetIndexedValue(&mut self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn CanWrite(&mut self) -> ::windows::core::Result<bool>;
    fn CanRead(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for ICustomProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomProperty";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ICustomProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>() -> ICustomProperty_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute(&target), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetIndexedValue<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndexedValue(::core::mem::transmute(&target), ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexedValue<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndexedValue(::core::mem::transmute(&target), ::core::mem::transmute(&value), ::core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn CanWrite<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomProperty, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIndexedValue: GetIndexedValue::<Identity, Impl, OFFSET>,
            SetIndexedValue: SetIndexedValue::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait ICustomPropertyProvider_Impl: Sized {
    fn GetCustomProperty(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ICustomProperty>;
    fn GetIndexedProperty(&mut self, name: &::windows::core::HSTRING, r#type: &super::Interop::TypeName) -> ::windows::core::Result<ICustomProperty>;
    fn GetStringRepresentation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&mut self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for ICustomPropertyProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ICustomPropertyProvider";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ICustomPropertyProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>() -> ICustomPropertyProvider_Vtbl {
        unsafe extern "system" fn GetCustomProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCustomProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexedProperty<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndexedProperty(::core::mem::transmute(&name), ::core::mem::transmute(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringRepresentation<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStringRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomPropertyProvider, OFFSET>(),
            GetCustomProperty: GetCustomProperty::<Identity, Impl, OFFSET>,
            GetIndexedProperty: GetIndexedProperty::<Identity, Impl, OFFSET>,
            GetStringRepresentation: GetStringRepresentation::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomPropertyProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IItemsRangeInfo_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn RangesChanged(&mut self, visiblerange: &::core::option::Option<ItemIndexRange>, trackeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IItemsRangeInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IItemsRangeInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IItemsRangeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsRangeInfo_Impl, const OFFSET: isize>() -> IItemsRangeInfo_Vtbl {
        unsafe extern "system" fn RangesChanged<Identity: ::windows::core::IUnknownImpl, Impl: IItemsRangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visiblerange: ::windows::core::RawPtr, trackeditems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RangesChanged(::core::mem::transmute(&visiblerange), ::core::mem::transmute(&trackeditems)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsRangeInfo, OFFSET>(), RangesChanged: RangesChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsRangeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait INotifyPropertyChanged_Impl: Sized {
    fn PropertyChanged(&mut self, handler: &::core::option::Option<PropertyChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for INotifyPropertyChanged {
    const NAME: &'static str = "Windows.UI.Xaml.Data.INotifyPropertyChanged";
}
#[cfg(feature = "Foundation")]
impl INotifyPropertyChanged_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>() -> INotifyPropertyChanged_Vtbl {
        unsafe extern "system" fn PropertyChanged<Identity: ::windows::core::IUnknownImpl, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Identity: ::windows::core::IUnknownImpl, Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePropertyChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotifyPropertyChanged, OFFSET>(),
            PropertyChanged: PropertyChanged::<Identity, Impl, OFFSET>,
            RemovePropertyChanged: RemovePropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotifyPropertyChanged as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ISelectionInfo_Impl: Sized {
    fn SelectRange(&mut self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn DeselectRange(&mut self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn IsSelected(&mut self, index: i32) -> ::windows::core::Result<bool>;
    fn GetSelectedRanges(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ISelectionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISelectionInfo";
}
#[cfg(feature = "Foundation_Collections")]
impl ISelectionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const OFFSET: isize>() -> ISelectionInfo_Vtbl {
        unsafe extern "system" fn SelectRange<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectRange(::core::mem::transmute(&itemindexrange)).into()
        }
        unsafe extern "system" fn DeselectRange<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeselectRange(::core::mem::transmute(&itemindexrange)).into()
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSelected(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedRanges<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelectedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionInfo, OFFSET>(),
            SelectRange: SelectRange::<Identity, Impl, OFFSET>,
            DeselectRange: DeselectRange::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            GetSelectedRanges: GetSelectedRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISupportIncrementalLoading_Impl: Sized {
    fn LoadMoreItemsAsync(&mut self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
    fn HasMoreItems(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISupportIncrementalLoading {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ISupportIncrementalLoading";
}
#[cfg(feature = "Foundation")]
impl ISupportIncrementalLoading_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>() -> ISupportIncrementalLoading_Vtbl {
        unsafe extern "system" fn LoadMoreItemsAsync<Identity: ::windows::core::IUnknownImpl, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Identity: ::windows::core::IUnknownImpl, Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISupportIncrementalLoading, OFFSET>(),
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportIncrementalLoading as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IValueConverter_Impl: Sized {
    fn Convert(&mut self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ConvertBack(&mut self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IValueConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Data.IValueConverter";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IValueConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueConverter_Impl, const OFFSET: isize>() -> IValueConverter_Vtbl {
        unsafe extern "system" fn Convert<Identity: ::windows::core::IUnknownImpl, Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Convert(::core::mem::transmute(&value), ::core::mem::transmute(&targettype), ::core::mem::transmute(&parameter), ::core::mem::transmute(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBack<Identity: ::windows::core::IUnknownImpl, Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertBack(::core::mem::transmute(&value), ::core::mem::transmute(&targettype), ::core::mem::transmute(&parameter), ::core::mem::transmute(&language)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IValueConverter, OFFSET>(),
            Convert: Convert::<Identity, Impl, OFFSET>,
            ConvertBack: ConvertBack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueConverter as ::windows::core::Interface>::IID
    }
}
