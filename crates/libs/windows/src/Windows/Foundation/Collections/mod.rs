#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl windows_core::TypeKind for CollectionChange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CollectionChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Collections.CollectionChange;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IIterable<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IIterable<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
pub trait IIterable_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<IIterator<T>>;
}
impl<T: windows_core::RuntimeType + 'static> IIterable_Vtbl<T> {
    pub const fn new<Identity: IIterable_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn First<T: windows_core::RuntimeType + 'static, Identity: IIterable_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterable_Impl::First(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterable<T>, OFFSET>(),
            First: First::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterable<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IIterator<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IIterator<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({6a79e863-4300-459a-9966-cbb660963ee1}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
pub trait IIterator_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Current(&self) -> windows_core::Result<T>;
    fn HasCurrent(&self) -> windows_core::Result<bool>;
    fn MoveNext(&self) -> windows_core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: IIterator_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Current<T: windows_core::RuntimeType + 'static, Identity: IIterator_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::Current(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<T: windows_core::RuntimeType + 'static, Identity: IIterator_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::HasCurrent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<T: windows_core::RuntimeType + 'static, Identity: IIterator_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: IIterator_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, items_array_size: u32, items: *mut T, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::GetMany(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterator<T>, OFFSET>(),
            Current: Current::<T, Identity, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, OFFSET>,
            MoveNext: MoveNext::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterator<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut T, *mut u32) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.Current().ok();
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IKeyValuePair<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IKeyValuePair<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IKeyValuePair<K, V> {}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IKeyValuePair<K, V> {
    type Vtable = IKeyValuePair_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IKeyValuePair<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IKeyValuePair<K, V> {
    pub fn Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<V> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IKeyValuePair<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IKeyValuePair";
}
pub trait IKeyValuePair_Impl<K, V>: windows_core::IUnknownImpl
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Key(&self) -> windows_core::Result<K>;
    fn Value(&self) -> windows_core::Result<V>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IKeyValuePair_Vtbl<K, V> {
    pub const fn new<Identity: IKeyValuePair_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<K>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKeyValuePair_Impl::Key(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IKeyValuePair_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKeyValuePair_Impl::Value(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyValuePair<K, V>, OFFSET>(),
            Key: Key::<K, V, Identity, OFFSET>,
            Value: Value::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyValuePair<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IKeyValuePair_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<K>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IMap<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMap<K, V> {}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IMap<K, V> {
    type Vtable = IMap_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMap<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMap<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<K, V>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
        P1: windows_core::Param<V>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), key.param().abi(), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), key.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
pub trait IMap_Impl<K, V>: IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<bool>;
    fn GetView(&self) -> windows_core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as windows_core::Type<K>>::Default, value: &<V as windows_core::Type<V>>::Default) -> windows_core::Result<bool>;
    fn Remove(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMap_Vtbl<K, V> {
    pub const fn new<Identity: IMap_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Lookup(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::HasKey(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::GetView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, value: windows_core::AbiType<V>, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Insert(this, core::mem::transmute(&key), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMap_Impl::Remove(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn Clear<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMap_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMap<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, OFFSET>,
            Size: Size::<K, V, Identity, OFFSET>,
            HasKey: HasKey::<K, V, Identity, OFFSET>,
            GetView: GetView::<K, V, Identity, OFFSET>,
            Insert: Insert::<K, V, Identity, OFFSET>,
            Remove: Remove::<K, V, Identity, OFFSET>,
            Clear: Clear::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMap<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IMap_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut bool) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, windows_core::AbiType<V>, *mut bool) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IMapChangedEventArgs<K>(windows_core::IUnknown, core::marker::PhantomData<K>)
where
    K: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMapChangedEventArgs<K> {}
impl<K: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMapChangedEventArgs<K> {}
unsafe impl<K: windows_core::RuntimeType + 'static> windows_core::Interface for IMapChangedEventArgs<K> {
    type Vtable = IMapChangedEventArgs_Vtbl<K>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMapChangedEventArgs<K> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9939f4df-050a-4c0f-aa60-77075f9c4777}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static> IMapChangedEventArgs<K> {
    pub fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CollectionChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMapChangedEventArgs<K> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapChangedEventArgs";
}
pub trait IMapChangedEventArgs_Impl<K>: windows_core::IUnknownImpl
where
    K: windows_core::RuntimeType + 'static,
{
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange>;
    fn Key(&self) -> windows_core::Result<K>;
}
impl<K: windows_core::RuntimeType + 'static> IMapChangedEventArgs_Vtbl<K> {
    pub const fn new<Identity: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CollectionChange<K: windows_core::RuntimeType + 'static, Identity: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CollectionChange) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapChangedEventArgs_Impl::CollectionChange(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<K: windows_core::RuntimeType + 'static, Identity: IMapChangedEventArgs_Impl<K>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<K>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapChangedEventArgs_Impl::Key(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMapChangedEventArgs<K>, OFFSET>(),
            CollectionChange: CollectionChange::<K, Identity, OFFSET>,
            Key: Key::<K, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapChangedEventArgs<K> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IMapChangedEventArgs_Vtbl<K>
where
    K: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CollectionChange) -> windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<K>) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IMapView<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMapView<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMapView<K, V> {}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IMapView<K, V> {
    type Vtable = IMapView_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMapView<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({e480ce40-a338-4ada-adcf-272272e48cb9}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMapView<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMapView<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Split(&self, first: &mut Option<IMapView<K, V>>, second: &mut Option<IMapView<K, V>>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMapView<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
pub trait IMapView_Impl<K, V>: IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<bool>;
    fn Split(&self, first: &mut Option<IMapView<K, V>>, second: &mut Option<IMapView<K, V>>) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMapView_Vtbl<K, V> {
    pub const fn new<Identity: IMapView_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::Lookup(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::HasKey(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IMapView_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, first: *mut *mut core::ffi::c_void, second: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapView_Impl::Split(this, core::mem::transmute_copy(&first), core::mem::transmute_copy(&second)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMapView<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, OFFSET>,
            Size: Size::<K, V, Identity, OFFSET>,
            HasKey: HasKey::<K, V, Identity, OFFSET>,
            Split: Split::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapView<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IMapView_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut bool) -> windows_core::HRESULT,
    pub Split: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IObservableMap<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IObservableMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IObservableMap<K, V> {}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IObservableMap<K, V> {
    type Vtable = IObservableMap_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IObservableMap<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({65df2bf5-bf39-41b5-aebc-5a9d865e472b}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IObservableMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IMap<K, V>> for IObservableMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IObservableMap<K, V> {
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<K, V>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<K, V>> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
        P1: windows_core::Param<V>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), key.param().abi(), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), key.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IObservableMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableMap";
}
pub trait IObservableMap_Impl<K, V>: IIterable_Impl<IKeyValuePair<K, V>> + IMap_Impl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn MapChanged(&self, vhnd: Option<&MapChangedEventHandler<K, V>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IObservableMap_Vtbl<K, V> {
    pub const fn new<Identity: IObservableMap_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapChanged<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, vhnd: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObservableMap_Impl::MapChanged(this, windows_core::from_raw_borrowed(&vhnd)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapChanged<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: IObservableMap_Impl<K, V>, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObservableMap_Impl::RemoveMapChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IObservableMap<K, V>, OFFSET>(),
            MapChanged: MapChanged::<K, V, Identity, OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObservableMap<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IObservableMap_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub MapChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMapChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IObservableVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IObservableVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IObservableVector<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IObservableVector<T> {
    type Vtable = IObservableVector_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IObservableVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({5917eb53-50b4-4a0d-b309-65862b3f1dbc}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IObservableVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IVector<T>> for IObservableVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IObservableVector<T> {
    pub fn VectorChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<VectorChangedEventHandler<T>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VectorChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveVectorChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVectorChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IVectorView<T>> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IObservableVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IObservableVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IObservableVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
}
pub trait IObservableVector_Impl<T>: IIterable_Impl<T> + IVector_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: Option<&VectorChangedEventHandler<T>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> IObservableVector_Vtbl<T> {
    pub const fn new<Identity: IObservableVector_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VectorChanged<T: windows_core::RuntimeType + 'static, Identity: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, vhnd: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObservableVector_Impl::VectorChanged(this, windows_core::from_raw_borrowed(&vhnd)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<T: windows_core::RuntimeType + 'static, Identity: IObservableVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObservableVector_Impl::RemoveVectorChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IObservableVector<T>, OFFSET>(),
            VectorChanged: VectorChanged::<T, Identity, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObservableVector<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IObservableVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub VectorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(IPropertySet, IPropertySet_Vtbl, 0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
impl windows_core::RuntimeType for IPropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPropertySet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( IPropertySet , IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , IMap < windows_core::HSTRING , windows_core::IInspectable > , IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl IPropertySet {
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl IntoIterator for IPropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &IPropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl windows_core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
pub trait IPropertySet_Impl: IIterable_Impl<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>> + IMap_Impl<windows_core::HSTRING, windows_core::IInspectable> + IObservableMap_Impl<windows_core::HSTRING, windows_core::IInspectable> {}
impl IPropertySet_Vtbl {
    pub const fn new<Identity: IPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertySet, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySet as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IPropertySet_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IVector<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IVectorView<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
pub trait IVector_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn GetView(&self) -> windows_core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as windows_core::Type<T>>::Default, index: &mut u32) -> windows_core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Append(&self, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn RemoveAtEnd(&self) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetMany(&self, startIndex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: IVector_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetAt(this, index) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::IndexOf(this, core::mem::transmute(&value), core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::SetAt(this, index, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::InsertAt(this, index, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::RemoveAt(this, index).into()
        }
        unsafe extern "system" fn Append<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::Append(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::RemoveAtEnd(this).into()
        }
        unsafe extern "system" fn Clear<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut T, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetMany(this, startindex, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAll<T: windows_core::RuntimeType + 'static, Identity: IVector_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, items_array_size: u32, items: *const T) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::ReplaceAll(this, core::slice::from_raw_parts(core::mem::transmute_copy(&items), items_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVector<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            GetView: GetView::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            SetAt: SetAt::<T, Identity, OFFSET>,
            InsertAt: InsertAt::<T, Identity, OFFSET>,
            RemoveAt: RemoveAt::<T, Identity, OFFSET>,
            Append: Append::<T, Identity, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Identity, OFFSET>,
            Clear: Clear::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            ReplaceAll: ReplaceAll::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVector<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>, *mut u32, *mut bool) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut T, *mut u32) -> windows_core::HRESULT,
    pub ReplaceAll: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const T) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(IVectorChangedEventArgs, IVectorChangedEventArgs_Vtbl, 0x575933df_34fe_4480_af15_07691f3d5d9b);
impl windows_core::RuntimeType for IVectorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IVectorChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IVectorChangedEventArgs {
    pub fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CollectionChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Index(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Index)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IVectorChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorChangedEventArgs";
}
pub trait IVectorChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange>;
    fn Index(&self) -> windows_core::Result<u32>;
}
impl IVectorChangedEventArgs_Vtbl {
    pub const fn new<Identity: IVectorChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CollectionChange<Identity: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CollectionChange) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorChangedEventArgs_Impl::CollectionChange(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: IVectorChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorChangedEventArgs_Impl::Index(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVectorChangedEventArgs, OFFSET>(),
            CollectionChange: CollectionChange::<Identity, OFFSET>,
            Index: Index::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVectorChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IVectorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CollectionChange) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IVectorView<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IVectorView<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IVectorView<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
pub trait IVectorView_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn IndexOf(&self, value: &<T as windows_core::Type<T>>::Default, index: &mut u32) -> windows_core::Result<bool>;
    fn GetMany(&self, startIndex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: IVectorView_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<T: windows_core::RuntimeType + 'static, Identity: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::GetAt(this, index) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: windows_core::RuntimeType + 'static, Identity: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: windows_core::RuntimeType + 'static, Identity: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::IndexOf(this, core::mem::transmute(&value), core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: IVectorView_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut T, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::GetMany(this, startindex, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVectorView<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVectorView<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IVectorView_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>, *mut u32, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut T, *mut u32) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapChangedEventHandler<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for MapChangedEventHandler<K, V> {
    type Vtable = MapChangedEventHandler_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for MapChangedEventHandler<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({179517f3-94ee-41f8-bddc-768a895544f3}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> MapChangedEventHandler<K, V> {
    pub fn new<F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MapChangedEventHandlerBox { vtable: &MapChangedEventHandlerBox::<K, V, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObservableMap<K, V>>,
        P1: windows_core::Param<IMapChangedEventArgs<K>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), event.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct MapChangedEventHandler_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(C)]
struct MapChangedEventHandlerBox<K, V, F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    vtable: *const MapChangedEventHandler_Vtbl<K, V>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static> MapChangedEventHandlerBox<K, V, F> {
    const VTABLE: MapChangedEventHandler_Vtbl<K, V> = MapChangedEventHandler_Vtbl::<K, V> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        K: core::marker::PhantomData::<K>,
        V: core::marker::PhantomData::<V>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MapChangedEventHandler<K, V> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&event)).into()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertySet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PropertySet, windows_core::IUnknown, windows_core::IInspectable, IPropertySet);
windows_core::imp::required_hierarchy ! ( PropertySet , IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , IMap < windows_core::HSTRING , windows_core::IInspectable > , IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl PropertySet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PropertySet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
}
unsafe impl windows_core::Interface for PropertySet {
    type Vtable = <IPropertySet as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.PropertySet";
}
unsafe impl Send for PropertySet {}
unsafe impl Sync for PropertySet {}
impl IntoIterator for PropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy ! ( StringMap , windows_core::IUnknown , windows_core::IInspectable , IMap < windows_core::HSTRING , windows_core::HSTRING > );
windows_core::imp::required_hierarchy ! ( StringMap , IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > > , IObservableMap < windows_core::HSTRING , windows_core::HSTRING > );
impl StringMap {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StringMap, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for StringMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMap<windows_core::HSTRING, windows_core::HSTRING>>();
}
unsafe impl windows_core::Interface for StringMap {
    type Vtable = <IMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StringMap {
    const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
}
unsafe impl Send for StringMap {}
unsafe impl Sync for StringMap {}
impl IntoIterator for StringMap {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &StringMap {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueSet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValueSet, windows_core::IUnknown, windows_core::IInspectable, IPropertySet);
windows_core::imp::required_hierarchy ! ( ValueSet , IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , IMap < windows_core::HSTRING , windows_core::IInspectable > , IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl ValueSet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValueSet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ValueSet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
}
unsafe impl windows_core::Interface for ValueSet {
    type Vtable = <IPropertySet as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ValueSet {
    const NAME: &'static str = "Windows.Foundation.Collections.ValueSet";
}
unsafe impl Send for ValueSet {}
unsafe impl Sync for ValueSet {}
impl IntoIterator for ValueSet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &ValueSet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorChangedEventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for VectorChangedEventHandler<T> {
    type Vtable = VectorChangedEventHandler_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for VectorChangedEventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({0c051752-9fbf-4c70-aa0c-0e4c82d9a761}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> VectorChangedEventHandler<T> {
    pub fn new<F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = VectorChangedEventHandlerBox { vtable: &VectorChangedEventHandlerBox::<T, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObservableVector<T>>,
        P1: windows_core::Param<IVectorChangedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), event.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct VectorChangedEventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(C)]
struct VectorChangedEventHandlerBox<T, F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>
where
    T: windows_core::RuntimeType + 'static,
{
    vtable: *const VectorChangedEventHandler_Vtbl<T>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<T: windows_core::RuntimeType + 'static, F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static> VectorChangedEventHandlerBox<T, F> {
    const VTABLE: VectorChangedEventHandler_Vtbl<T> = VectorChangedEventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <VectorChangedEventHandler<T> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&event)).into()
    }
}
