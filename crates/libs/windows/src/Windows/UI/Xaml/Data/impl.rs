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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICollectionView_Vtbl {
        unsafe extern "system" fn CurrentItem<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPosition<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAfterLast<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAfterLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentBeforeFirst<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentBeforeFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionGroups<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentChanged(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentChanging<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentChanging(&*(&handler as *const <CurrentChangingEventHandler as ::windows::core::Abi>::Abi as *const <CurrentChangingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanging<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentChanging(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MoveCurrentTo<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentTo(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPosition<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToFirst<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToLast<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToLast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToNext<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPrevious<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveCurrentToPrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMoreItemsAsync<Impl: ICollectionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionView, BASE_OFFSET>(),
            CurrentItem: CurrentItem::<Impl, IMPL_OFFSET>,
            CurrentPosition: CurrentPosition::<Impl, IMPL_OFFSET>,
            IsCurrentAfterLast: IsCurrentAfterLast::<Impl, IMPL_OFFSET>,
            IsCurrentBeforeFirst: IsCurrentBeforeFirst::<Impl, IMPL_OFFSET>,
            CollectionGroups: CollectionGroups::<Impl, IMPL_OFFSET>,
            HasMoreItems: HasMoreItems::<Impl, IMPL_OFFSET>,
            CurrentChanged: CurrentChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentChanged: RemoveCurrentChanged::<Impl, IMPL_OFFSET>,
            CurrentChanging: CurrentChanging::<Impl, IMPL_OFFSET>,
            RemoveCurrentChanging: RemoveCurrentChanging::<Impl, IMPL_OFFSET>,
            MoveCurrentTo: MoveCurrentTo::<Impl, IMPL_OFFSET>,
            MoveCurrentToPosition: MoveCurrentToPosition::<Impl, IMPL_OFFSET>,
            MoveCurrentToFirst: MoveCurrentToFirst::<Impl, IMPL_OFFSET>,
            MoveCurrentToLast: MoveCurrentToLast::<Impl, IMPL_OFFSET>,
            MoveCurrentToNext: MoveCurrentToNext::<Impl, IMPL_OFFSET>,
            MoveCurrentToPrevious: MoveCurrentToPrevious::<Impl, IMPL_OFFSET>,
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICollectionViewFactory_Vtbl {
        unsafe extern "system" fn CreateView<Impl: ICollectionViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionViewFactory, BASE_OFFSET>(), CreateView: CreateView::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICollectionViewGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICollectionViewGroup_Vtbl {
        unsafe extern "system" fn Group<Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupItems<Impl: ICollectionViewGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICollectionViewGroup, BASE_OFFSET>(),
            Group: Group::<Impl, IMPL_OFFSET>,
            GroupItems: GroupItems::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomProperty_Vtbl {
        unsafe extern "system" fn Type<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIndexedValue<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexedValue(&*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&index as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexedValue<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetIndexedValue(
                    &*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&index as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CanWrite<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Impl: ICustomProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomProperty, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetIndexedValue: GetIndexedValue::<Impl, IMPL_OFFSET>,
            SetIndexedValue: SetIndexedValue::<Impl, IMPL_OFFSET>,
            CanWrite: CanWrite::<Impl, IMPL_OFFSET>,
            CanRead: CanRead::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomPropertyProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomPropertyProvider_Vtbl {
        unsafe extern "system" fn GetCustomProperty<Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomProperty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexedProperty<Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexedProperty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&r#type as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringRepresentation<Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICustomPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomPropertyProvider, BASE_OFFSET>(),
            GetCustomProperty: GetCustomProperty::<Impl, IMPL_OFFSET>,
            GetIndexedProperty: GetIndexedProperty::<Impl, IMPL_OFFSET>,
            GetStringRepresentation: GetStringRepresentation::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsRangeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsRangeInfo_Vtbl {
        unsafe extern "system" fn RangesChanged<Impl: IItemsRangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visiblerange: ::windows::core::RawPtr, trackeditems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RangesChanged(&*(&visiblerange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType), &*(&trackeditems as *const <super::super::super::Foundation::Collections::IVectorView<ItemIndexRange> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<ItemIndexRange> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsRangeInfo, BASE_OFFSET>(), RangesChanged: RangesChanged::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotifyPropertyChanged_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotifyPropertyChanged_Vtbl {
        unsafe extern "system" fn PropertyChanged<Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyChanged(&*(&handler as *const <PropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <PropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Impl: INotifyPropertyChanged_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertyChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotifyPropertyChanged, BASE_OFFSET>(),
            PropertyChanged: PropertyChanged::<Impl, IMPL_OFFSET>,
            RemovePropertyChanged: RemovePropertyChanged::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionInfo_Vtbl {
        unsafe extern "system" fn SelectRange<Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectRange(&*(&itemindexrange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeselectRange<Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeselectRange(&*(&itemindexrange as *const <ItemIndexRange as ::windows::core::Abi>::Abi as *const <ItemIndexRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSelected<Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedRanges<Impl: ISelectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectionInfo, BASE_OFFSET>(),
            SelectRange: SelectRange::<Impl, IMPL_OFFSET>,
            DeselectRange: DeselectRange::<Impl, IMPL_OFFSET>,
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            GetSelectedRanges: GetSelectedRanges::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportIncrementalLoading_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportIncrementalLoading_Vtbl {
        unsafe extern "system" fn LoadMoreItemsAsync<Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<Impl: ISupportIncrementalLoading_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISupportIncrementalLoading, BASE_OFFSET>(),
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Impl, IMPL_OFFSET>,
            HasMoreItems: HasMoreItems::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueConverter_Vtbl {
        unsafe extern "system" fn Convert<Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Convert(
                &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&targettype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBack<Impl: IValueConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertBack(
                &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&targettype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IValueConverter, BASE_OFFSET>(),
            Convert: Convert::<Impl, IMPL_OFFSET>,
            ConvertBack: ConvertBack::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueConverter as ::windows::core::Interface>::IID
    }
}
