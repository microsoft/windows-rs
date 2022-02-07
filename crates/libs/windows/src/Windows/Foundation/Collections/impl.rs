pub trait IIterable_Impl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn First(&self) -> ::windows::core::Result<IIterator<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
impl<T: ::windows::core::RuntimeType + 'static> IIterable_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIterable_Impl<T>, const OFFSET: isize>() -> IIterable_Vtbl<T> {
        unsafe extern "system" fn First<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IIterable_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIterable<T>, OFFSET>(),
            First: First::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterable<T> as ::windows::core::Interface>::IID
    }
}
pub trait IIterator_Impl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Current(&self) -> ::windows::core::Result<T>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as ::windows::core::RuntimeType>::DefaultType]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
impl<T: ::windows::core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIterator_Impl<T>, const OFFSET: isize>() -> IIterator_Vtbl<T> {
        unsafe extern "system" fn Current<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasCurrent<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveNext<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIterator<T>, OFFSET>(),
            Current: Current::<T, Identity, Impl, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterator<T> as ::windows::core::Interface>::IID
    }
}
pub trait IKeyValuePair_Impl<K, V>: Sized
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
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IKeyValuePair_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>() -> IKeyValuePair_Vtbl<K, V> {
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyValuePair<K, V>, OFFSET>(),
            Key: Key::<K, V, Identity, Impl, OFFSET>,
            Value: Value::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyValuePair<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<bool>;
    fn GetView(&self) -> ::windows::core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType, value: &<V as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Remove(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMap_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>() -> IMap_Vtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Lookup(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasKey(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Insert<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, value: <V as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Insert(::core::mem::transmute(&key), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn Clear<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMap<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, Impl, OFFSET>,
            Size: Size::<K, V, Identity, Impl, OFFSET>,
            HasKey: HasKey::<K, V, Identity, Impl, OFFSET>,
            GetView: GetView::<K, V, Identity, Impl, OFFSET>,
            Insert: Insert::<K, V, Identity, Impl, OFFSET>,
            Remove: Remove::<K, V, Identity, Impl, OFFSET>,
            Clear: Clear::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMap<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IMapChangedEventArgs_Impl<K>: Sized
where
    K: ::windows::core::RuntimeType + 'static,
{
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Key(&self) -> ::windows::core::Result<K>;
}
impl<K: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMapChangedEventArgs<K> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapChangedEventArgs";
}
impl<K: ::windows::core::RuntimeType + 'static> IMapChangedEventArgs_Vtbl<K> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>() -> IMapChangedEventArgs_Vtbl<K> {
        unsafe extern "system" fn CollectionChange<K: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <K as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapChangedEventArgs<K>, OFFSET>(),
            CollectionChange: CollectionChange::<K, Identity, Impl, OFFSET>,
            Key: Key::<K, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapChangedEventArgs<K> as ::windows::core::Interface>::IID
    }
}
pub trait IMapView_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<bool>;
    fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMapView<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMapView_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapView_Impl<K, V>, const OFFSET: isize>() -> IMapView_Vtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut <V as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Lookup(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: <K as ::windows::core::Abi>::Abi, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HasKey(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: *mut ::windows::core::RawPtr, second: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Split(::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapView<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, Impl, OFFSET>,
            Size: Size::<K, V, Identity, Impl, OFFSET>,
            HasKey: HasKey::<K, V, Identity, Impl, OFFSET>,
            Split: Split::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapView<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IObservableMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>> + IMap_Impl<K, V>
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
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IObservableMap_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>() -> IObservableMap_Vtbl<K, V> {
        unsafe extern "system" fn MapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MapChanged(::core::mem::transmute(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMapChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObservableMap<K, V>, OFFSET>(),
            MapChanged: MapChanged::<K, V, Identity, Impl, OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableMap<K, V> as ::windows::core::Interface>::IID
    }
}
pub trait IObservableVector_Impl<T>: Sized + IIterable_Impl<T> + IVector_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: &::core::option::Option<VectorChangedEventHandler<T>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IObservableVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IObservableVector_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObservableVector_Impl<T>, const OFFSET: isize>() -> IObservableVector_Vtbl<T> {
        unsafe extern "system" fn VectorChanged<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VectorChanged(::core::mem::transmute(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveVectorChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IObservableVector<T>, OFFSET>(),
            VectorChanged: VectorChanged::<T, Identity, Impl, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableVector<T> as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySet_Impl: Sized + IIterable_Impl<IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMap_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> + IObservableMap_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
impl ::windows::core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
impl IPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySet_Impl, const OFFSET: isize>() -> IPropertySet_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertySet, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySet as ::windows::core::Interface>::IID
    }
}
pub trait IVector_Impl<T>: Sized + IIterable_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as ::windows::core::RuntimeType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::RuntimeType>::DefaultType]) -> ::windows::core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as ::windows::core::RuntimeType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>() -> IVector_Vtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetView<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Append(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMany(startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAll<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *const <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReplaceAll(::core::slice::from_raw_parts(::core::mem::transmute_copy(&items), items_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, Impl, OFFSET>,
            Size: Size::<T, Identity, Impl, OFFSET>,
            GetView: GetView::<T, Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<T, Identity, Impl, OFFSET>,
            SetAt: SetAt::<T, Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<T, Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<T, Identity, Impl, OFFSET>,
            Append: Append::<T, Identity, Impl, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Identity, Impl, OFFSET>,
            Clear: Clear::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            ReplaceAll: ReplaceAll::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector<T> as ::windows::core::Interface>::IID
    }
}
pub trait IVectorChangedEventArgs_Impl: Sized {
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Index(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IVectorChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorChangedEventArgs";
}
impl IVectorChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>() -> IVectorChangedEventArgs_Vtbl {
        unsafe extern "system" fn CollectionChange<Identity: ::windows::core::IUnknownImpl, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVectorChangedEventArgs, OFFSET>(),
            CollectionChange: CollectionChange::<Identity, Impl, OFFSET>,
            Index: Index::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorChangedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IVectorView_Impl<T>: Sized + IIterable_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &<T as ::windows::core::RuntimeType>::DefaultType, index: &mut u32) -> ::windows::core::Result<bool>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::RuntimeType>::DefaultType]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
impl<T: ::windows::core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVectorView_Impl<T>, const OFFSET: isize>() -> IVectorView_Vtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: <T as ::windows::core::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::core::Abi>::Abi, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVectorView<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, Impl, OFFSET>,
            Size: Size::<T, Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorView<T> as ::windows::core::Interface>::IID
    }
}
