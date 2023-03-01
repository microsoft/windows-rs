#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterable_Impl<T>, const OFFSET: isize>() -> IIterable_Vtbl<T> {
        unsafe extern "system" fn First<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterable_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.First() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IIterable<T>, OFFSET>(),
            First: First::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterable<T> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IIterator_Impl<T>: Sized
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Current(&self) -> ::windows::core::Result<T>;
    fn HasCurrent(&self) -> ::windows::core::Result<bool>;
    fn MoveNext(&self) -> ::windows::core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as ::windows::core::Type<T>>::Default]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
impl<T: ::windows::core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: isize>() -> IIterator_Vtbl<T> {
        unsafe extern "system" fn Current<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Current() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *mut ::windows::core::AbiType<T>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMany(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IIterator<T>, OFFSET>(),
            Current: Current::<T, Identity, Impl, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIterator<T> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>() -> IKeyValuePair_Vtbl<K, V> {
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::AbiType<K>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Key() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::AbiType<V>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IKeyValuePair<K, V>, OFFSET>(),
            Key: Key::<K, V, Identity, Impl, OFFSET>,
            Value: Value::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyValuePair<K, V> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::Type<K>>::Default) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::Type<K>>::Default) -> ::windows::core::Result<bool>;
    fn GetView(&self) -> ::windows::core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as ::windows::core::Type<K>>::Default, value: &<V as ::windows::core::Type<V>>::Default) -> ::windows::core::Result<bool>;
    fn Remove(&self, key: &<K as ::windows::core::Type<K>>::Default) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMap_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>() -> IMap_Vtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>, result__: *mut ::windows::core::AbiType<V>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lookup(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasKey(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>, value: ::windows::core::AbiType<V>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Insert(::core::mem::transmute(&key), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn Clear<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMap<K, V>, OFFSET>(),
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
        iid == &<IMap<K, V> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>() -> IMapChangedEventArgs_Vtbl<K> {
        unsafe extern "system" fn CollectionChange<K: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<K: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::AbiType<K>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Key() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMapChangedEventArgs<K>, OFFSET>(),
            CollectionChange: CollectionChange::<K, Identity, Impl, OFFSET>,
            Key: Key::<K, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapChangedEventArgs<K> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IMapView_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as ::windows::core::Type<K>>::Default) -> ::windows::core::Result<V>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn HasKey(&self, key: &<K as ::windows::core::Type<K>>::Default) -> ::windows::core::Result<bool>;
    fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IMapView<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IMapView_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: isize>() -> IMapView_Vtbl<K, V> {
        unsafe extern "system" fn Lookup<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>, result__: *mut ::windows::core::AbiType<V>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lookup(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::AbiType<K>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasKey(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void, second: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Split(::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMapView<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, Impl, OFFSET>,
            Size: Size::<K, V, Identity, Impl, OFFSET>,
            HasKey: HasKey::<K, V, Identity, Impl, OFFSET>,
            Split: Split::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapView<K, V> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IObservableMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>> + IMap_Impl<K, V>
where
    K: ::windows::core::RuntimeType + 'static,
    V: ::windows::core::RuntimeType + 'static,
{
    fn MapChanged(&self, vhnd: ::core::option::Option<&MapChangedEventHandler<K, V>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IObservableMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableMap";
}
impl<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static> IObservableMap_Vtbl<K, V> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>() -> IObservableMap_Vtbl<K, V> {
        unsafe extern "system" fn MapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapChanged(::windows::core::from_raw_borrowed(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapChanged<K: ::windows::core::RuntimeType + 'static, V: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMapChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IObservableMap<K, V>, OFFSET>(),
            MapChanged: MapChanged::<K, V, Identity, Impl, OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableMap<K, V> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IObservableVector_Impl<T>: Sized + IIterable_Impl<T> + IVector_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: ::core::option::Option<&VectorChangedEventHandler<T>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IObservableVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IObservableVector_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: isize>() -> IObservableVector_Vtbl<T> {
        unsafe extern "system" fn VectorChanged<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VectorChanged(::windows::core::from_raw_borrowed(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveVectorChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IObservableVector<T>, OFFSET>(),
            VectorChanged: VectorChanged::<T, Identity, Impl, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObservableVector<T> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IPropertySet_Impl: Sized + IIterable_Impl<IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMap_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> + IObservableMap_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
impl ::windows::core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
impl IPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPropertySet_Impl, const OFFSET: isize>() -> IPropertySet_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPropertySet, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySet as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IVector_Impl<T>: Sized + IIterable_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn GetView(&self) -> ::windows::core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as ::windows::core::Type<T>>::Default, index: &mut u32) -> ::windows::core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as ::windows::core::Type<T>>::Default) -> ::windows::core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as ::windows::core::Type<T>>::Default) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()>;
    fn Append(&self, value: &<T as ::windows::core::Type<T>>::Default) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::Type<T>>::Default]) -> ::windows::core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as ::windows::core::Type<T>>::Default]) -> ::windows::core::Result<()>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
impl<T: ::windows::core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>() -> IVector_Vtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexOf(::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(index, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows::core::AbiType<T>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMany(startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAll<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *const ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplaceAll(::core::slice::from_raw_parts(::core::mem::transmute_copy(&items), items_array_size as _)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IVector<T>, OFFSET>(),
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
        iid == &<IVector<T> as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IVectorChangedEventArgs_Impl: Sized {
    fn CollectionChange(&self) -> ::windows::core::Result<CollectionChange>;
    fn Index(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IVectorChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorChangedEventArgs";
}
impl IVectorChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>() -> IVectorChangedEventArgs_Vtbl {
        unsafe extern "system" fn CollectionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Index() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IVectorChangedEventArgs, OFFSET>(),
            CollectionChange: CollectionChange::<Identity, Impl, OFFSET>,
            Index: Index::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorChangedEventArgs as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Collections\"`, `\"implement\"`*"]
pub trait IVectorView_Impl<T>: Sized + IIterable_Impl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> ::windows::core::Result<T>;
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn IndexOf(&self, value: &<T as ::windows::core::Type<T>>::Default, index: &mut u32) -> ::windows::core::Result<bool>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::core::Type<T>>::Default]) -> ::windows::core::Result<u32>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
impl<T: ::windows::core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: isize>() -> IVectorView_Vtbl<T> {
        unsafe extern "system" fn GetAt<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows::core::AbiType<T>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexOf(::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows::core::AbiType<T>, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMany(startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IVectorView<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, Impl, OFFSET>,
            Size: Size::<T, Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVectorView<T> as ::windows::core::ComInterface>::IID
    }
}
#[::windows::core::implement(IIterable<T>)]
struct StockIterable<T>
where
    T: ::windows::core::RuntimeType + 'static,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockIterable<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    fn First(&self) -> ::windows::core::Result<IIterator<T>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(StockIterator { owner: self.cast()?, current: 0.into() }.into())
        }
    }
}

#[::windows::core::implement(IIterator<T>)]
struct StockIterator<T>
where
    T: ::windows::core::RuntimeType + 'static,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    owner: IIterable<T>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockIterator<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    fn Current(&self) -> ::windows::core::Result<T> {
        let owner: &StockIterable<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if owner.values.len() > current {
            T::from_default(&owner.values[current])
        } else {
            Err(::windows::imp::E_BOUNDS.into())
        }
    }

    fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let owner: &StockIterable<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(owner.values.len() > current)
    }

    fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let owner: &StockIterable<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < owner.values.len() {
            self.current.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> ::windows::core::Result<u32> {
        let owner: &StockIterable<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&owner.values[current..current + actual]);
        self.current.fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as _)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IIterable<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    type Error = ::windows::core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows::core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(StockIterable { values }.into())
    }
}
#[windows::core::implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: windows::core::RuntimeType + 'static,
    V: windows::core::RuntimeType + 'static,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView<K, V>
where
    K: windows::core::RuntimeType,
    V: windows::core::RuntimeType,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    fn First(&self) -> windows::core::Result<IIterator<IKeyValuePair<K, V>>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(StockMapViewIterator::<K, V> { _owner: self.cast()?, current: std::sync::RwLock::new(self.map.iter()) }.into())
        }
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView<K, V>
where
    K: windows::core::RuntimeType,
    V: windows::core::RuntimeType,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    fn Lookup(&self, key: &K::Default) -> windows::core::Result<V> {
        let value = self.map.get(key).ok_or_else(|| windows::core::Error::from(windows::imp::E_BOUNDS))?;
        V::from_default(value)
    }
    fn Size(&self) -> windows::core::Result<u32> {
        Ok(self.map.len() as _)
    }
    fn HasKey(&self, key: &K::Default) -> windows::core::Result<bool> {
        Ok(self.map.contains_key(key))
    }
    fn Split(&self, first: &mut std::option::Option<IMapView<K, V>>, second: &mut std::option::Option<IMapView<K, V>>) -> windows::core::Result<()> {
        *first = None;
        *second = None;
        Ok(())
    }
}

#[::windows::core::implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<'a, K, V>
where
    K: windows::core::RuntimeType + 'static,
    V: windows::core::RuntimeType + 'static,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    _owner: IIterable<IKeyValuePair<K, V>>,
    current: ::std::sync::RwLock<std::collections::btree_map::Iter<'a, K::Default, V::Default>>,
}

impl<'a, K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator<'a, K, V>
where
    K: windows::core::RuntimeType,
    V: windows::core::RuntimeType,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    fn Current(&self) -> ::windows::core::Result<IKeyValuePair<K, V>> {
        let mut current = self.current.read().unwrap().clone().peekable();

        if let Some((key, value)) = current.peek() {
            Ok(StockKeyValuePair { key: (*key).clone(), value: (*value).clone() }.into())
        } else {
            Err(windows::core::Error::from(windows::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let mut current = self.current.read().unwrap().clone().peekable();

        Ok(current.peek().is_some())
    }

    fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let mut current = self.current.write().unwrap();

        current.next();
        Ok(current.clone().peekable().peek().is_some())
    }

    fn GetMany(&self, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> ::windows::core::Result<u32> {
        let mut current = self.current.write().unwrap();
        let mut actual = 0;

        for pair in pairs {
            if let Some((key, value)) = current.next() {
                *pair = Some(StockKeyValuePair { key: (*key).clone(), value: (*value).clone() }.into());
                actual += 1;
            } else {
                break;
            }
        }

        Ok(actual as _)
    }
}

#[windows::core::implement(IKeyValuePair<K, V>)]
struct StockKeyValuePair<K, V>
where
    K: windows::core::RuntimeType + 'static,
    V: windows::core::RuntimeType + 'static,
    <K as windows::core::Type<K>>::Default: std::clone::Clone,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    key: K::Default,
    value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair<K, V>
where
    K: windows::core::RuntimeType,
    V: windows::core::RuntimeType,
    <K as windows::core::Type<K>>::Default: std::clone::Clone,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    fn Key(&self) -> windows::core::Result<K> {
        K::from_default(&self.key)
    }
    fn Value(&self) -> windows::core::Result<V> {
        V::from_default(&self.value)
    }
}

impl<K, V> ::core::convert::TryFrom<std::collections::BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K: windows::core::RuntimeType,
    V: windows::core::RuntimeType,
    <K as windows::core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as windows::core::Type<V>>::Default: std::clone::Clone,
{
    type Error = ::windows::core::Error;
    fn try_from(map: std::collections::BTreeMap<K::Default, V::Default>) -> windows::core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(StockMapView { map }.into())
    }
}
#[windows::core::implement(IVectorView<T>, IIterable<T>)]
struct StockVectorView<T>
where
    T: windows::core::RuntimeType + 'static,
    <T as windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockVectorView<T>
where
    T: windows::core::RuntimeType,
    <T as windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn First(&self) -> windows::core::Result<IIterator<T>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(StockVectorViewIterator { owner: self.cast()?, current: 0.into() }.into())
        }
    }
}

impl<T> IVectorView_Impl<T> for StockVectorView<T>
where
    T: windows::core::RuntimeType,
    <T as windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn GetAt(&self, index: u32) -> windows::core::Result<T> {
        let item = self.values.get(index as usize).ok_or_else(|| windows::core::Error::from(windows::imp::E_BOUNDS))?;
        T::from_default(item)
    }
    fn Size(&self) -> windows::core::Result<u32> {
        Ok(self.values.len() as _)
    }
    fn IndexOf(&self, value: &T::Default, result: &mut u32) -> windows::core::Result<bool> {
        match self.values.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as _;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    fn GetMany(&self, current: u32, values: &mut [T::Default]) -> windows::core::Result<u32> {
        let current = current as usize;
        if current >= self.values.len() {
            return Ok(0);
        }
        let actual = std::cmp::min(self.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&self.values[current..current + actual]);
        Ok(actual as _)
    }
}

#[::windows::core::implement(IIterator<T>)]
struct StockVectorViewIterator<T>
where
    T: ::windows::core::RuntimeType + 'static,
    <T as ::windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    owner: IIterable<T>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockVectorViewIterator<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn Current(&self) -> ::windows::core::Result<T> {
        let owner: &StockVectorView<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if owner.values.len() > current {
            T::from_default(&owner.values[current])
        } else {
            Err(windows::core::Error::from(windows::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let owner: &StockVectorView<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(owner.values.len() > current)
    }

    fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let owner: &StockVectorView<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < owner.values.len() {
            self.current.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> ::windows::core::Result<u32> {
        let owner: &StockVectorView<T> = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&owner.values[current..current + actual]);
        self.current.fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as _)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IVectorView<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    type Error = ::windows::core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows::core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(StockVectorView { values }.into())
    }
}
