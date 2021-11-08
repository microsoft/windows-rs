#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Foundation_Collections`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: CollectionChange = CollectionChange(0i32);
    pub const ItemInserted: CollectionChange = CollectionChange(1i32);
    pub const ItemRemoved: CollectionChange = CollectionChange(2i32);
    pub const ItemChanged: CollectionChange = CollectionChange(3i32);
}
impl ::core::convert::From<i32> for CollectionChange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CollectionChange {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CollectionChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Collections.CollectionChange;i4)");
}
impl ::windows::runtime::DefaultType for CollectionChange {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IIterable<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IIterable<T> {
    type Vtable = IIterable_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IIterable<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IIterable<T> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<T>>(result__)
        }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IIterable<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IIterable<T>> for ::windows::runtime::IUnknown {
    fn from(value: IIterable<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IIterable<T>> for ::windows::runtime::IUnknown {
    fn from(value: &IIterable<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIterable<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IIterable<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IIterable<T>> for ::windows::runtime::IInspectable {
    fn from(value: IIterable<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IIterable<T>> for ::windows::runtime::IInspectable {
    fn from(value: &IIterable<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IIterable<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IIterable<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterable_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IIterator<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IIterator<T> {
    type Vtable = IIterator_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IIterator<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IIterator<T> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<T> {
        let this = self;
        unsafe {
            let mut result__: <T as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IIterator<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IIterator<T>> for ::windows::runtime::IUnknown {
    fn from(value: IIterator<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IIterator<T>> for ::windows::runtime::IUnknown {
    fn from(value: &IIterator<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIterator<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IIterator<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IIterator<T>> for ::windows::runtime::IInspectable {
    fn from(value: IIterator<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IIterator<T>> for ::windows::runtime::IInspectable {
    fn from(value: &IIterator<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IIterator<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IIterator<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        let result = self.Current().ok();
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterator_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, items_array_size: u32, items: *mut <T as ::windows::runtime::Abi>::Abi, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IKeyValuePair<K, V>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IKeyValuePair<K, V> {
    type Vtable = IKeyValuePair_abi<K, V>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IKeyValuePair<K, V> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> IKeyValuePair<K, V> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<K> {
        let this = self;
        unsafe {
            let mut result__: <K as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<K>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<V> {
        let this = self;
        unsafe {
            let mut result__: <V as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<V>(result__)
        }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IKeyValuePair<K, V> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{02b51929-c1c4-4a7e-8940-0312b5c18500}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IKeyValuePair<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: IKeyValuePair<K, V>) -> Self {
        value.0 .0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IKeyValuePair<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: &IKeyValuePair<K, V>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKeyValuePair<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKeyValuePair<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IKeyValuePair<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: IKeyValuePair<K, V>) -> Self {
        value.0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IKeyValuePair<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: &IKeyValuePair<K, V>) -> Self {
        value.0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IKeyValuePair<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IKeyValuePair<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyValuePair_abi<K, V>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <K as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <V as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
    pub ::core::marker::PhantomData<V>,
)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IMap<K, V>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IMap<K, V> {
    type Vtable = IMap_abi<K, V>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IMap<K, V> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> IMap<K, V> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<V> {
        let this = self;
        unsafe {
            let mut result__: <V as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<V>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<K, V>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<K, V>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, K>, Param1: ::windows::runtime::IntoParam<'a, V>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<K, V>>>(result__)
        }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IMap<K, V> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{3c2925fe-8519-45c1-aa79-197b6718c1c1}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMap<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: IMap<K, V>) -> Self {
        value.0 .0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMap<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: &IMap<K, V>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMap<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: IMap<K, V>) -> Self {
        value.0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMap<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: &IMap<K, V>) -> Self {
        value.0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IMap<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IMap<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for &IMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<K, V>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for &IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMap_abi<K, V>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi, result__: *mut <V as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi, value: <V as ::windows::runtime::Abi>::Abi, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
    pub ::core::marker::PhantomData<V>,
)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IMapChangedEventArgs<K>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<K>)
where
    K: ::windows::runtime::RuntimeType + 'static;
unsafe impl<K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IMapChangedEventArgs<K> {
    type Vtable = IMapChangedEventArgs_abi<K>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IMapChangedEventArgs<K> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<K: ::windows::runtime::RuntimeType + 'static> IMapChangedEventArgs<K> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn CollectionChange(&self) -> ::windows::runtime::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__: CollectionChange = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CollectionChange>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<K> {
        let this = self;
        unsafe {
            let mut result__: <K as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<K>(result__)
        }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IMapChangedEventArgs<K> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9939f4df-050a-4c0f-aa60-77075f9c4777}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMapChangedEventArgs<K>> for ::windows::runtime::IUnknown {
    fn from(value: IMapChangedEventArgs<K>) -> Self {
        value.0 .0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMapChangedEventArgs<K>> for ::windows::runtime::IUnknown {
    fn from(value: &IMapChangedEventArgs<K>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMapChangedEventArgs<K> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMapChangedEventArgs<K> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMapChangedEventArgs<K>> for ::windows::runtime::IInspectable {
    fn from(value: IMapChangedEventArgs<K>) -> Self {
        value.0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMapChangedEventArgs<K>> for ::windows::runtime::IInspectable {
    fn from(value: &IMapChangedEventArgs<K>) -> Self {
        value.0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMapChangedEventArgs<K> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMapChangedEventArgs<K> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapChangedEventArgs_abi<K>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CollectionChange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <K as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
)
where
    K: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IMapView<K, V>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IMapView<K, V> {
    type Vtable = IMapView_abi<K, V>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IMapView<K, V> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> IMapView<K, V> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<V> {
        let this = self;
        unsafe {
            let mut result__: <V as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<V>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Split(&self, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<K, V>>>(result__)
        }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IMapView<K, V> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{e480ce40-a338-4ada-adcf-272272e48cb9}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMapView<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: IMapView<K, V>) -> Self {
        value.0 .0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMapView<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: &IMapView<K, V>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IMapView<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: IMapView<K, V>) -> Self {
        value.0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IMapView<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: &IMapView<K, V>) -> Self {
        value.0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IMapView<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMapView<K, V>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IMapView<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMapView<K, V>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for &IMapView<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<K, V>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for &IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapView_abi<K, V>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi, result__: *mut <V as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: <K as ::windows::runtime::Abi>::Abi, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, first: *mut ::windows::runtime::RawPtr, second: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
    pub ::core::marker::PhantomData<V>,
)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IObservableMap<K, V>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IObservableMap<K, V> {
    type Vtable = IObservableMap_abi<K, V>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IObservableMap<K, V> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> IObservableMap<K, V> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, MapChangedEventHandler<K, V>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<K, V>>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<V> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__: <V as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<V>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<K, V>> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<K, V>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, K>, Param1: ::windows::runtime::IntoParam<'a, V>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, K>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IObservableMap<K, V> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{65df2bf5-bf39-41b5-aebc-5a9d865e472b}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IObservableMap<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: IObservableMap<K, V>) -> Self {
        value.0 .0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IObservableMap<K, V>> for ::windows::runtime::IUnknown {
    fn from(value: &IObservableMap<K, V>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IObservableMap<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: IObservableMap<K, V>) -> Self {
        value.0
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IObservableMap<K, V>> for ::windows::runtime::IInspectable {
    fn from(value: &IObservableMap<K, V>) -> Self {
        value.0.clone()
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IObservableMap<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IObservableMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IObservableMap<K, V>> for IIterable<IKeyValuePair<K, V>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IObservableMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<K, V>>> for &IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<K, V>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<K, V>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IObservableMap<K, V>> for IMap<K, V> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IObservableMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IObservableMap<K, V>> for IMap<K, V> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IObservableMap<K, V>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IMap<K, V>> for IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<K, V>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IMap<K, V>> for &IObservableMap<K, V> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<K, V>> {
        ::core::convert::TryInto::<IMap<K, V>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for &IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObservableMap_abi<K, V>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vhnd: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
    pub ::core::marker::PhantomData<V>,
)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IObservableVector<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IObservableVector<T> {
    type Vtable = IObservableVector_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IObservableVector<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IObservableVector<T> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn VectorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, VectorChangedEventHandler<T>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveVectorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<T>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<T>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<T> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__: <T as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IVectorView<T>> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IVectorView<T>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, T>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, T>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, T>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, T>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVector<T>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IObservableVector<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{5917eb53-50b4-4a0d-b309-65862b3f1dbc}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IObservableVector<T>> for ::windows::runtime::IUnknown {
    fn from(value: IObservableVector<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IObservableVector<T>> for ::windows::runtime::IUnknown {
    fn from(value: &IObservableVector<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IObservableVector<T>> for ::windows::runtime::IInspectable {
    fn from(value: IObservableVector<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IObservableVector<T>> for ::windows::runtime::IInspectable {
    fn from(value: &IObservableVector<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IObservableVector<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IObservableVector<T>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IObservableVector<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IObservableVector<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for &IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::core::convert::TryInto::<IIterable<T>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IObservableVector<T>> for IVector<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IObservableVector<T>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IObservableVector<T>> for IVector<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IObservableVector<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IVector<T>> for IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVector<T>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IVector<T>> for &IObservableVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVector<T>> {
        ::core::convert::TryInto::<IVector<T>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::iter::IntoIterator for &IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObservableVector_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vhnd: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IPropertySet(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPropertySet {
    type Vtable = IPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319707551, 62694, 17441, [172, 249, 29, 171, 41, 134, 130, 12]);
}
impl IPropertySet {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPropertySet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8a43ed9f-f4e6-4421-acf9-1dab2986820c}");
}
impl ::core::convert::From<IPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySet) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySet) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPropertySet> for ::windows::runtime::IInspectable {
    fn from(value: IPropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertySet> for ::windows::runtime::IInspectable {
    fn from(value: &IPropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPropertySet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPropertySet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IPropertySet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPropertySet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IPropertySet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPropertySet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &IPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for IPropertySet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &IPropertySet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IVector<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IVector<T> {
    type Vtable = IVector_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IVector<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IVector<T> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<T> {
        let this = self;
        unsafe {
            let mut result__: <T as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IVectorView<T>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IVectorView<T>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, T>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, T>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, T>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, T>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<T>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<T>>(result__)
        }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IVector<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{913337e9-11a1-4345-a3a2-4e7f956e222d}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IVector<T>> for ::windows::runtime::IUnknown {
    fn from(value: IVector<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IVector<T>> for ::windows::runtime::IUnknown {
    fn from(value: &IVector<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IVector<T>> for ::windows::runtime::IInspectable {
    fn from(value: IVector<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IVector<T>> for ::windows::runtime::IInspectable {
    fn from(value: &IVector<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IVector<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IVector<T>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IVector<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IVector<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for &IVector<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::core::convert::TryInto::<IIterable<T>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
pub struct VectorIterator<T: ::windows::runtime::RuntimeType + 'static> {
    vector: ::core::option::Option<IVector<T>>,
    current: u32,
}
impl<T: ::windows::runtime::RuntimeType> VectorIterator<T> {
    pub fn new(vector: ::core::option::Option<IVector<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::Iterator for VectorIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: <T as ::windows::runtime::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::runtime::Abi>::Abi, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, items_array_size: u32, items: *const <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IVectorChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVectorChangedEventArgs {
    type Vtable = IVectorChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1465463775, 13566, 17536, [175, 21, 7, 105, 31, 61, 93, 155]);
}
impl IVectorChangedEventArgs {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn CollectionChange(&self) -> ::windows::runtime::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__: CollectionChange = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CollectionChange>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Index(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVectorChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{575933df-34fe-4480-af15-07691f3d5d9b}");
}
impl ::core::convert::From<IVectorChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IVectorChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVectorChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IVectorChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVectorChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVectorChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVectorChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IVectorChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVectorChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IVectorChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVectorChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVectorChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CollectionChange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Collections`*"]
pub struct IVectorView<T>(pub ::windows::runtime::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IVectorView<T> {
    type Vtable = IVectorView_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IVectorView<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IVectorView<T> {
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<T> {
        let this = self;
        unsafe {
            let mut result__: <T as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, T>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<T>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<T>>(result__)
        }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IVectorView<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IVectorView<T>> for ::windows::runtime::IUnknown {
    fn from(value: IVectorView<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IVectorView<T>> for ::windows::runtime::IUnknown {
    fn from(value: &IVectorView<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<IVectorView<T>> for ::windows::runtime::IInspectable {
    fn from(value: IVectorView<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::From<&IVectorView<T>> for ::windows::runtime::IInspectable {
    fn from(value: &IVectorView<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<IVectorView<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IVectorView<T>) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::core::convert::TryFrom<&IVectorView<T>> for IIterable<T> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IVectorView<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IIterable<T>> for &IVectorView<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<T>> {
        ::core::convert::TryInto::<IIterable<T>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
pub struct VectorViewIterator<T: ::windows::runtime::RuntimeType + 'static> {
    vector: ::core::option::Option<IVectorView<T>>,
    current: u32,
}
impl<T: ::windows::runtime::RuntimeType> VectorViewIterator<T> {
    pub fn new(vector: ::core::option::Option<IVectorView<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::Iterator for VectorViewIterator<T> {
    type Item = T;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl<T: ::windows::runtime::RuntimeType> ::core::iter::IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorViewIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorView_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: <T as ::windows::runtime::Abi>::Abi, index: *mut u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startindex: u32, items_array_size: u32, items: *mut <T as ::windows::runtime::Abi>::Abi, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[doc = "*Required features: `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MapChangedEventHandler<K, V>(::windows::runtime::IUnknown, ::core::marker::PhantomData<K>, ::core::marker::PhantomData<V>)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> MapChangedEventHandler<K, V> {
    pub fn new<F: FnMut(&::core::option::Option<IObservableMap<K, V>>, &::core::option::Option<IMapChangedEventArgs<K>>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MapChangedEventHandler_box::<K, V, F> {
            vtable: &MapChangedEventHandler_box::<K, V, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IObservableMap<K, V>>, Param1: ::windows::runtime::IntoParam<'a, IMapChangedEventArgs<K>>>(&self, sender: Param0, event: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), event.into_param().abi()).ok() }
    }
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for MapChangedEventHandler<K, V> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{179517f3-94ee-41f8-bddc-768a895544f3}").push_slice(b";").push_other(<K as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<V as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for MapChangedEventHandler<K, V> {
    type Vtable = MapChangedEventHandler_abi<K, V>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<MapChangedEventHandler<K, V> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct MapChangedEventHandler_abi<K, V>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<K>,
    pub ::core::marker::PhantomData<V>,
)
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct MapChangedEventHandler_box<K, V, F: FnMut(&::core::option::Option<IObservableMap<K, V>>, &::core::option::Option<IMapChangedEventArgs<K>>) -> ::windows::runtime::Result<()> + 'static>
where
    K: ::windows::runtime::RuntimeType + 'static,
    V: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const MapChangedEventHandler_abi<K, V>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<K: ::windows::runtime::RuntimeType + 'static, V: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::core::option::Option<IObservableMap<K, V>>, &::core::option::Option<IMapChangedEventArgs<K>>) -> ::windows::runtime::Result<()> + 'static> MapChangedEventHandler_box<K, V, F> {
    const VTABLE: MapChangedEventHandler_abi<K, V> = MapChangedEventHandler_abi::<K, V>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<K>, ::core::marker::PhantomData::<V>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MapChangedEventHandler<K, V> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <IObservableMap<K, V> as ::windows::runtime::Abi>::Abi as *const <IObservableMap<K, V> as ::windows::runtime::DefaultType>::DefaultType),
            &*(&event as *const <IMapChangedEventArgs<K> as ::windows::runtime::Abi>::Abi as *const <IMapChangedEventArgs<K> as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PropertySet(pub ::windows::runtime::IInspectable);
impl PropertySet {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PropertySet, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PropertySet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.PropertySet;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
unsafe impl ::windows::runtime::Interface for PropertySet {
    type Vtable = IPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319707551, 62694, 17441, [172, 249, 29, 171, 41, 134, 130, 12]);
}
impl ::windows::runtime::RuntimeName for PropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.PropertySet";
}
impl ::core::convert::From<PropertySet> for ::windows::runtime::IUnknown {
    fn from(value: PropertySet) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &PropertySet) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PropertySet> for ::windows::runtime::IInspectable {
    fn from(value: PropertySet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PropertySet> for ::windows::runtime::IInspectable {
    fn from(value: &PropertySet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PropertySet> for IPropertySet {
    fn from(value: PropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertySet> for IPropertySet {
    fn from(value: &PropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertySet> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertySet> for &PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PropertySet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PropertySet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<PropertySet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PropertySet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<PropertySet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PropertySet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PropertySet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &PropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PropertySet {}
unsafe impl ::core::marker::Sync for PropertySet {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PropertySet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PropertySet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StringMap(pub ::windows::runtime::IInspectable);
impl StringMap {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StringMap, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StringMap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.StringMap;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;string))");
}
unsafe impl ::windows::runtime::Interface for StringMap {
    type Vtable = IMap_abi<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl ::windows::runtime::RuntimeName for StringMap {
    const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
}
impl ::core::convert::From<StringMap> for ::windows::runtime::IUnknown {
    fn from(value: StringMap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StringMap> for ::windows::runtime::IUnknown {
    fn from(value: &StringMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StringMap> for ::windows::runtime::IInspectable {
    fn from(value: StringMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StringMap> for ::windows::runtime::IInspectable {
    fn from(value: &StringMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<StringMap> for IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: StringMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StringMap> for IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    fn from(value: &StringMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StringMap> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StringMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StringMap> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StringMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<StringMap> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StringMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StringMap> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StringMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &StringMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::core::convert::TryInto::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for StringMap {}
unsafe impl ::core::marker::Sync for StringMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for StringMap {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &StringMap {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ValueSet(pub ::windows::runtime::IInspectable);
impl ValueSet {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ValueSet, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IIterator<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ValueSet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Collections.ValueSet;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
unsafe impl ::windows::runtime::Interface for ValueSet {
    type Vtable = IPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319707551, 62694, 17441, [172, 249, 29, 171, 41, 134, 130, 12]);
}
impl ::windows::runtime::RuntimeName for ValueSet {
    const NAME: &'static str = "Windows.Foundation.Collections.ValueSet";
}
impl ::core::convert::From<ValueSet> for ::windows::runtime::IUnknown {
    fn from(value: ValueSet) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ValueSet> for ::windows::runtime::IUnknown {
    fn from(value: &ValueSet) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ValueSet> for ::windows::runtime::IInspectable {
    fn from(value: ValueSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ValueSet> for ::windows::runtime::IInspectable {
    fn from(value: &ValueSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ValueSet> for IPropertySet {
    fn from(value: ValueSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValueSet> for IPropertySet {
    fn from(value: &ValueSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertySet> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertySet> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertySet> for &ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertySet> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ValueSet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ValueSet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ValueSet> for IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ValueSet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::core::convert::TryInto::<IIterable<IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ValueSet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ValueSet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ValueSet> for IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ValueSet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ValueSet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ValueSet) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ValueSet> for IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ValueSet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &ValueSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::core::convert::TryInto::<IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ValueSet {}
unsafe impl ::core::marker::Sync for ValueSet {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ValueSet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ValueSet {
    type Item = IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VectorChangedEventHandler<T>(::windows::runtime::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
impl<T: ::windows::runtime::RuntimeType + 'static> VectorChangedEventHandler<T> {
    pub fn new<F: FnMut(&::core::option::Option<IObservableVector<T>>, &::core::option::Option<IVectorChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = VectorChangedEventHandler_box::<T, F> {
            vtable: &VectorChangedEventHandler_box::<T, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IObservableVector<T>>, Param1: ::windows::runtime::IntoParam<'a, IVectorChangedEventArgs>>(&self, sender: Param0, event: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), event.into_param().abi()).ok() }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for VectorChangedEventHandler<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{0c051752-9fbf-4c70-aa0c-0e4c82d9a761}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for VectorChangedEventHandler<T> {
    type Vtable = VectorChangedEventHandler_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<VectorChangedEventHandler<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct VectorChangedEventHandler_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct VectorChangedEventHandler_box<T, F: FnMut(&::core::option::Option<IObservableVector<T>>, &::core::option::Option<IVectorChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static>
where
    T: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const VectorChangedEventHandler_abi<T>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<T: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::core::option::Option<IObservableVector<T>>, &::core::option::Option<IVectorChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static> VectorChangedEventHandler_box<T, F> {
    const VTABLE: VectorChangedEventHandler_abi<T> = VectorChangedEventHandler_abi::<T>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<T>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<VectorChangedEventHandler<T> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, event: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <IObservableVector<T> as ::windows::runtime::Abi>::Abi as *const <IObservableVector<T> as ::windows::runtime::DefaultType>::DefaultType),
            &*(&event as *const <IVectorChangedEventArgs as ::windows::runtime::Abi>::Abi as *const <IVectorChangedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
