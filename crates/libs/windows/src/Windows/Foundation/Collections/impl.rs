pub trait IIterableImpl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn First(&self) -> ::windows::core::Result<IIterator<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
impl<T: ::windows::core::RuntimeType + 'static> IIterableVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIterableImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIterableVtbl<T> {
        unsafe extern "system" fn First<T: ::windows::core::RuntimeType + 'static, Impl: IIterableImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIterable<T>, BASE_OFFSET>(),
            First: First::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterable<T> as ::windows::core::Interface>::IID
    }
}
pub trait IIteratorImpl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Current(&self) -> ::windows::core::Result<T>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
impl<T: ::windows::core::RuntimeType + 'static> IIteratorVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIteratorImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIteratorVtbl<T> {
        unsafe extern "system" fn Current<T: ::windows::core::RuntimeType + 'static, Impl: IIteratorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasCurrent<T: ::windows::core::RuntimeType + 'static, Impl: IIteratorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveNext<T: ::windows::core::RuntimeType + 'static, Impl: IIteratorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Impl: IIteratorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMany(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIterator<T>, BASE_OFFSET>(),
            Current: Current::<T, Impl, IMPL_OFFSET>,
            HasCurrent: HasCurrent::<T, Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<T, Impl, IMPL_OFFSET>,
            GetMany: GetMany::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterator<T> as ::windows::core::Interface>::IID
    }
}
pub trait IKeyValuePairImpl<K, V>: Sized
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Key(&self) -> ::windows::core::Result<K>;
    fn Value(&self) -> ::windows::core::Result<V>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IKeyValuePair<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IKeyValuePair";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IKeyValuePairVtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyValuePairImpl<K, V>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyValuePairVtbl<K, V> {
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IKeyValuePairImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IKeyValuePairImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyValuePair<K, V>, BASE_OFFSET>(),
            Key: Key::<K, V, Impl, IMPL_OFFSET>,
            Value: Value::<K, V, Impl, IMPL_OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyValuePair<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IMapImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn GetView(&self) -> ::windows::core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as ::windows::core::DefaultType>::DefaultType, value: &<V as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Remove(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMapVtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapImpl<K, V>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapVtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKey(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Insert<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, value: <V as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <V as ::windows::core::Abi>::Abi as *const <V as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMap<K, V>, BASE_OFFSET>(),
            Lookup: Lookup::<K, V, Impl, IMPL_OFFSET>,
            Size: Size::<K, V, Impl, IMPL_OFFSET>,
            HasKey: HasKey::<K, V, Impl, IMPL_OFFSET>,
            GetView: GetView::<K, V, Impl, IMPL_OFFSET>,
            Insert: Insert::<K, V, Impl, IMPL_OFFSET>,
            Remove: Remove::<K, V, Impl, IMPL_OFFSET>,
            Clear: Clear::<K, V, Impl, IMPL_OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMap<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IMapChangedEventArgsImpl<K>: Sized
where
    K: ::windows::core::RuntimeType + 'static,
{
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Key(&self) -> ::windows::core::Result<K>;
}
impl<K: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMapChangedEventArgs<K> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapChangedEventArgs";
}
impl<K: ::windows::core::RuntimeType + 'static> IMapChangedEventArgsVtbl<K> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapChangedEventArgsImpl<K>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapChangedEventArgsVtbl<K> {
        unsafe extern "system" fn CollectionChange<K: ::windows::core::RuntimeType + 'static, Impl: IMapChangedEventArgsImpl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, Impl: IMapChangedEventArgsImpl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapChangedEventArgs<K>, BASE_OFFSET>(),
            CollectionChange: CollectionChange::<K, Impl, IMPL_OFFSET>,
            Key: Key::<K, Impl, IMPL_OFFSET>,
            K: ::core::marker::PhantomData::<K>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapChangedEventArgs<K> as ::windows::core::Interface>::IID
    }
}
pub trait IMapViewImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMapView<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMapViewVtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapViewImpl<K, V>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapViewVtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapViewImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapViewImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapViewImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKey(&*(&key as *const <K as ::windows::core::Abi>::Abi as *const <K as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IMapViewImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: *mut ::windows::core::RawPtr, second: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Split(::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapView<K, V>, BASE_OFFSET>(),
            Lookup: Lookup::<K, V, Impl, IMPL_OFFSET>,
            Size: Size::<K, V, Impl, IMPL_OFFSET>,
            HasKey: HasKey::<K, V, Impl, IMPL_OFFSET>,
            Split: Split::<K, V, Impl, IMPL_OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapView<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IObservableMapImpl<K, V>: Sized + IIterableImpl<IKeyValuePair<K, V>> + IMapImpl<K, V>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn MapChanged(&self, vhnd: &::core::option::Option<MapChangedEventHandler<K, V>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IObservableMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableMap";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IObservableMapVtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObservableMapImpl<K, V>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObservableMapVtbl<K, V> {
        unsafe extern "system" fn MapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IObservableMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapChanged(&*(&vhnd as *const <MapChangedEventHandler<K, V> as ::windows::core::Abi>::Abi as *const <MapChangedEventHandler<K, V> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Impl: IObservableMapImpl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapChanged(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObservableMap<K, V>, BASE_OFFSET>(),
            MapChanged: MapChanged::<K, V, Impl, IMPL_OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Impl, IMPL_OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableMap<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IObservableVectorImpl<T>: Sized + IIterableImpl<T> + IVectorImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: &::core::option::Option<VectorChangedEventHandler<T>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IObservableVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IObservableVectorVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObservableVectorImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObservableVectorVtbl<T> {
        unsafe extern "system" fn VectorChanged<T: ::windows::core::RuntimeType + 'static, Impl: IObservableVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VectorChanged(&*(&vhnd as *const <VectorChangedEventHandler<T> as ::windows::core::Abi>::Abi as *const <VectorChangedEventHandler<T> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<T: ::windows::core::RuntimeType + 'static, Impl: IObservableVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVectorChanged(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObservableVector<T>, BASE_OFFSET>(),
            VectorChanged: VectorChanged::<T, Impl, IMPL_OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableVector<T> as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySetImpl: Sized + IIterableImpl<IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> + IObservableMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
impl ::windows::core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
impl IPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySetVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertySet, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySet as ::windows::core::Interface>::IID
    }
}
pub trait IVectorImpl<T>: Sized + IIterableImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as ::windows::core::DefaultType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IVectorVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVectorImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVectorVtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetView<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexOf(&*(&value as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAt(index, &*(&value as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertAt<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(index, &*(&value as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAt<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&value as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMany(startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAll<T: ::windows::core::RuntimeType + 'static, Impl: IVectorImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *const <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceAll(::core::slice::from_raw_parts(::core::mem::transmute_copy(&items), items_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector<T>, BASE_OFFSET>(),
            GetAt: GetAt::<T, Impl, IMPL_OFFSET>,
            Size: Size::<T, Impl, IMPL_OFFSET>,
            GetView: GetView::<T, Impl, IMPL_OFFSET>,
            IndexOf: IndexOf::<T, Impl, IMPL_OFFSET>,
            SetAt: SetAt::<T, Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<T, Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<T, Impl, IMPL_OFFSET>,
            Append: Append::<T, Impl, IMPL_OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Impl, IMPL_OFFSET>,
            Clear: Clear::<T, Impl, IMPL_OFFSET>,
            GetMany: GetMany::<T, Impl, IMPL_OFFSET>,
            ReplaceAll: ReplaceAll::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector<T> as ::windows::core::Interface>::IID
    }
}
pub trait IVectorChangedEventArgsImpl: Sized {
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Index(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IVectorChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorChangedEventArgs";
}
impl IVectorChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVectorChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVectorChangedEventArgsVtbl {
        unsafe extern "system" fn CollectionChange<Impl: IVectorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Impl: IVectorChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVectorChangedEventArgs, BASE_OFFSET>(),
            CollectionChange: CollectionChange::<Impl, IMPL_OFFSET>,
            Index: Index::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorChangedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IVectorViewImpl<T>: Sized + IIterableImpl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &<T as ::windows::core::DefaultType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
impl<T: ::windows::core::RuntimeType + 'static> IVectorViewVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVectorViewImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVectorViewVtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Impl: IVectorViewImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Impl: IVectorViewImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Impl: IVectorViewImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexOf(&*(&value as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Impl: IVectorViewImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMany(startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVectorView<T>, BASE_OFFSET>(),
            GetAt: GetAt::<T, Impl, IMPL_OFFSET>,
            Size: Size::<T, Impl, IMPL_OFFSET>,
            IndexOf: IndexOf::<T, Impl, IMPL_OFFSET>,
            GetMany: GetMany::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorView<T> as ::windows::core::Interface>::IID
    }
}
