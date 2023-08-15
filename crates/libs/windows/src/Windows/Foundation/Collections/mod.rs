#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IIterable<T>(::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> ::windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IIterable<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IIterable<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IIterable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IIterable<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IIterable<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIterable").field(&self.0).finish()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IIterable<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IIterable<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterable_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IIterator<T>(::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> ::windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMany(&self, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IIterator<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IIterator<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IIterator<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IIterator<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIterator").field(&self.0).finish()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        let result = self.Current().ok();
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IIterator<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IIterator<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterator_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IKeyValuePair<K, V>(::windows_core::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> IKeyValuePair<K, V> {
    pub fn Key(&self) -> ::windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<V> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IKeyValuePair<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IKeyValuePair<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IKeyValuePair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IKeyValuePair<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IKeyValuePair<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKeyValuePair").field(&self.0).finish()
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IKeyValuePair<K, V> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{02b51929-c1c4-4a7e-8940-0312b5c18500}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IKeyValuePair<K, V> {
    type Vtable = IKeyValuePair_Vtbl<K, V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IKeyValuePair<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IKeyValuePair<K, V> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyValuePair_Vtbl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<K>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IMap<K, V>(::windows_core::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> IMap<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> ::windows_core::Result<V>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<K, V>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<K>,
        P1: ::windows_core::IntoParam<V>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<IKeyValuePair<K, V>>> for IMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IMap<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMap").field(&self.0).finish()
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IMap<K, V> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{3c2925fe-8519-45c1-aa79-197b6718c1c1}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for &IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IMap<K, V> {
    type Vtable = IMap_Vtbl<K, V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IMap<K, V> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMap_Vtbl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, value: ::windows_core::AbiType<V>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IMapChangedEventArgs<K>(::windows_core::IUnknown, ::core::marker::PhantomData<K>)
where
    K: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static> IMapChangedEventArgs<K> {
    pub fn CollectionChange(&self) -> ::windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CollectionChange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Key(&self) -> ::windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IMapChangedEventArgs<K> {}
impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IMapChangedEventArgs<K> {}
impl<K: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IMapChangedEventArgs<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IMapChangedEventArgs<K> {}
impl<K: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IMapChangedEventArgs<K> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapChangedEventArgs").field(&self.0).finish()
    }
}
impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IMapChangedEventArgs<K> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9939f4df-050a-4c0f-aa60-77075f9c4777}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IMapChangedEventArgs<K> {
    type Vtable = IMapChangedEventArgs_Vtbl<K>;
}
impl<K: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IMapChangedEventArgs<K> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IMapChangedEventArgs<K> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapChangedEventArgs_Vtbl<K>
where
    K: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<K>) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IMapView<K, V>(::windows_core::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> IMapView<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> ::windows_core::Result<V>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IMapView<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IMapView<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<IKeyValuePair<K, V>>> for IMapView<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IMapView<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IMapView<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IMapView<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMapView").field(&self.0).finish()
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IMapView<K, V> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{e480ce40-a338-4ada-adcf-272272e48cb9}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for &IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IMapView<K, V> {
    type Vtable = IMapView_Vtbl<K, V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IMapView<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IMapView<K, V> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapView_Vtbl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Split: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void, second: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IObservableMap<K, V>(::windows_core::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> IObservableMap<K, V> {
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MapChangedEventHandler<K, V>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Lookup<P0>(&self, key: P0) -> ::windows_core::Result<V>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<K, V>> {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<K>,
        P1: ::windows_core::IntoParam<V>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<K>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<K, V>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IObservableMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IObservableMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<IKeyValuePair<K, V>>> for IObservableMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IMap<K, V>> for IObservableMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IObservableMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IObservableMap<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IObservableMap<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObservableMap").field(&self.0).finish()
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IObservableMap<K, V> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{65df2bf5-bf39-41b5-aebc-5a9d865e472b}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for &IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IObservableMap<K, V> {
    type Vtable = IObservableMap_Vtbl<K, V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IObservableMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IObservableMap<K, V> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObservableMap_Vtbl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MapChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMapChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IObservableVector<T>(::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> IObservableVector<T> {
    pub fn VectorChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<VectorChangedEventHandler<T>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VectorChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveVectorChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVectorChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<T>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<T> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IVectorView<T>> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVector<T>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IObservableVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IObservableVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<T>> for IObservableVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IVector<T>> for IObservableVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IObservableVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IObservableVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IObservableVector<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObservableVector").field(&self.0).finish()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IObservableVector<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{5917eb53-50b4-4a0d-b309-65862b3f1dbc}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::iter::IntoIterator for &IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IObservableVector<T> {
    type Vtable = IObservableVector_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IObservableVector<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IObservableVector<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObservableVector_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VectorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IPropertySet(::windows_core::IUnknown);
impl IPropertySet {
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IPropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for IPropertySet {}
impl ::windows_core::CanTryInto<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for IPropertySet {}
impl ::windows_core::CanTryInto<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for IPropertySet {}
impl ::core::cmp::PartialEq for IPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySet {}
impl ::core::fmt::Debug for IPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySet").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IPropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{8a43ed9f-f4e6-4421-acf9-1dab2986820c}");
}
impl ::core::iter::IntoIterator for IPropertySet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &IPropertySet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl ::windows_core::Interface for IPropertySet {
    type Vtable = IPropertySet_Vtbl;
}
impl ::core::clone::Clone for IPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySet_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IVector<T>(::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IVectorView<T>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<T>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<T>> for IVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IVector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IVector<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IVector<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVector").field(&self.0).finish()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{913337e9-11a1-4345-a3a2-4e7f956e222d}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
pub struct VectorIterator<T: ::windows_core::RuntimeType + 'static> {
    vector: ::core::option::Option<IVector<T>>,
    current: u32,
}
impl<T: ::windows_core::RuntimeType> VectorIterator<T> {
    pub fn new(vector: ::core::option::Option<IVector<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for VectorIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IVector<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IVector<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReplaceAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *const ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IVectorChangedEventArgs(::windows_core::IUnknown);
impl IVectorChangedEventArgs {
    pub fn CollectionChange(&self) -> ::windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CollectionChange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Index(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Index)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVectorChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVectorChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVectorChangedEventArgs {}
impl ::core::fmt::Debug for IVectorChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVectorChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVectorChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{575933df-34fe-4480-af15-07691f3d5d9b}");
}
unsafe impl ::windows_core::Interface for IVectorChangedEventArgs {
    type Vtable = IVectorChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVectorChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVectorChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x575933df_34fe_4480_af15_07691f3d5d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct IVectorView<T>(::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> IVectorView<T> {
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<T>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IUnknown> for IVectorView<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanInto<::windows_core::IInspectable> for IVectorView<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::CanTryInto<IIterable<T>> for IVectorView<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for IVectorView<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for IVectorView<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for IVectorView<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVectorView").field(&self.0).finish()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
pub struct VectorViewIterator<T: ::windows_core::RuntimeType + 'static> {
    vector: ::core::option::Option<IVectorView<T>>,
    current: u32,
}
impl<T: ::windows_core::RuntimeType> VectorViewIterator<T> {
    pub fn new(vector: ::core::option::Option<IVectorView<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for VectorViewIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorViewIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for IVectorView<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for IVectorView<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorView_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct PropertySet(::windows_core::IUnknown);
impl PropertySet {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PropertySet, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for PropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertySet {}
impl ::core::fmt::Debug for PropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertySet").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.PropertySet;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
impl ::core::clone::Clone for PropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PropertySet {
    type Vtable = IPropertySet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertySet {
    const IID: ::windows_core::GUID = <IPropertySet as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.PropertySet";
}
impl ::core::iter::IntoIterator for PropertySet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &PropertySet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(PropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for PropertySet {}
impl ::windows_core::CanTryInto<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PropertySet {}
impl ::windows_core::CanTryInto<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PropertySet {}
impl ::windows_core::CanTryInto<IPropertySet> for PropertySet {}
unsafe impl ::core::marker::Send for PropertySet {}
unsafe impl ::core::marker::Sync for PropertySet {}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct StringMap(::windows_core::IUnknown);
impl StringMap {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StringMap, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert(&self, key: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for StringMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StringMap {}
impl ::core::fmt::Debug for StringMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StringMap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StringMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.StringMap;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;string))");
}
impl ::core::clone::Clone for StringMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StringMap {
    type Vtable = IMap_Vtbl<::windows_core::HSTRING, ::windows_core::HSTRING>;
}
unsafe impl ::windows_core::ComInterface for StringMap {
    const IID: ::windows_core::GUID = <IMap<::windows_core::HSTRING, ::windows_core::HSTRING> as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StringMap {
    const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
}
impl ::core::iter::IntoIterator for StringMap {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &StringMap {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(StringMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for StringMap {}
impl ::windows_core::CanTryInto<IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for StringMap {}
impl ::windows_core::CanTryInto<IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for StringMap {}
unsafe impl ::core::marker::Send for StringMap {}
unsafe impl ::core::marker::Sync for StringMap {}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct ValueSet(::windows_core::IUnknown);
impl ValueSet {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ValueSet, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for ValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValueSet {}
impl ::core::fmt::Debug for ValueSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueSet").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ValueSet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.ValueSet;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
impl ::core::clone::Clone for ValueSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ValueSet {
    type Vtable = IPropertySet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ValueSet {
    const IID: ::windows_core::GUID = <IPropertySet as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ValueSet {
    const NAME: &'static str = "Windows.Foundation.Collections.ValueSet";
}
impl ::core::iter::IntoIterator for ValueSet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &ValueSet {
    type Item = IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(ValueSet, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for ValueSet {}
impl ::windows_core::CanTryInto<IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for ValueSet {}
impl ::windows_core::CanTryInto<IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for ValueSet {}
impl ::windows_core::CanTryInto<IPropertySet> for ValueSet {}
unsafe impl ::core::marker::Send for ValueSet {}
unsafe impl ::core::marker::Sync for ValueSet {}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl ::core::marker::Copy for CollectionChange {}
impl ::core::clone::Clone for CollectionChange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CollectionChange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CollectionChange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CollectionChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CollectionChange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CollectionChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Collections.CollectionChange;i4)");
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct MapChangedEventHandler<K, V>(pub ::windows_core::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static;
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> MapChangedEventHandler<K, V> {
    pub fn new<F: FnMut(::core::option::Option<&IObservableMap<K, V>>, ::core::option::Option<&IMapChangedEventArgs<K>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MapChangedEventHandlerBox::<K, V, F> { vtable: &MapChangedEventHandlerBox::<K, V, F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IObservableMap<K, V>>,
        P1: ::windows_core::TryIntoParam<IMapChangedEventArgs<K>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.try_into_param()?.abi(), event.try_into_param()?.abi()).ok() }
    }
}
#[repr(C)]
struct MapChangedEventHandlerBox<K, V, F: FnMut(::core::option::Option<&IObservableMap<K, V>>, ::core::option::Option<&IMapChangedEventArgs<K>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    vtable: *const MapChangedEventHandler_Vtbl<K, V>,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, F: FnMut(::core::option::Option<&IObservableMap<K, V>>, ::core::option::Option<&IMapChangedEventArgs<K>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> MapChangedEventHandlerBox<K, V, F> {
    const VTABLE: MapChangedEventHandler_Vtbl<K, V> = MapChangedEventHandler_Vtbl::<K, V> {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        K: ::core::marker::PhantomData::<K>,
        V: ::core::marker::PhantomData::<V>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MapChangedEventHandler<K, V> as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&event)).into()
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for MapChangedEventHandler<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for MapChangedEventHandler<K, V> {}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for MapChangedEventHandler<K, V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for MapChangedEventHandler<K, V> {
    type Vtable = MapChangedEventHandler_Vtbl<K, V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for MapChangedEventHandler<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>)
    }
}
unsafe impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for MapChangedEventHandler<K, V> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for MapChangedEventHandler<K, V> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{179517f3-94ee-41f8-bddc-768a895544f3}").push_slice(b";").push_other(<K as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
#[repr(C)]
#[doc(hidden)]
pub struct MapChangedEventHandler_Vtbl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct VectorChangedEventHandler<T>(pub ::windows_core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows_core::RuntimeType + 'static;
impl<T: ::windows_core::RuntimeType + 'static> VectorChangedEventHandler<T> {
    pub fn new<F: FnMut(::core::option::Option<&IObservableVector<T>>, ::core::option::Option<&IVectorChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = VectorChangedEventHandlerBox::<T, F> { vtable: &VectorChangedEventHandlerBox::<T, F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IObservableVector<T>>,
        P1: ::windows_core::TryIntoParam<IVectorChangedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.try_into_param()?.abi(), event.try_into_param()?.abi()).ok() }
    }
}
#[repr(C)]
struct VectorChangedEventHandlerBox<T, F: FnMut(::core::option::Option<&IObservableVector<T>>, ::core::option::Option<&IVectorChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>
where
    T: ::windows_core::RuntimeType + 'static,
{
    vtable: *const VectorChangedEventHandler_Vtbl<T>,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<T: ::windows_core::RuntimeType + 'static, F: FnMut(::core::option::Option<&IObservableVector<T>>, ::core::option::Option<&IVectorChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> VectorChangedEventHandlerBox<T, F> {
    const VTABLE: VectorChangedEventHandler_Vtbl<T> = VectorChangedEventHandler_Vtbl::<T> {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        T: ::core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<VectorChangedEventHandler<T> as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&event)).into()
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::PartialEq for VectorChangedEventHandler<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::cmp::Eq for VectorChangedEventHandler<T> {}
impl<T: ::windows_core::RuntimeType + 'static> ::core::fmt::Debug for VectorChangedEventHandler<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VectorChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Interface for VectorChangedEventHandler<T> {
    type Vtable = VectorChangedEventHandler_Vtbl<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::core::clone::Clone for VectorChangedEventHandler<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
unsafe impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::ComInterface for VectorChangedEventHandler<T> {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_signature(<Self as ::windows_core::RuntimeType>::SIGNATURE);
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::RuntimeType for VectorChangedEventHandler<T> {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = { ::windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{0c051752-9fbf-4c70-aa0c-0e4c82d9a761}").push_slice(b";").push_other(<T as ::windows_core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
#[repr(C)]
#[doc(hidden)]
pub struct VectorChangedEventHandler_Vtbl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
