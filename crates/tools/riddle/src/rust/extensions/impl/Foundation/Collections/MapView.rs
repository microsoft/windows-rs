#[::windows_implement::implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn First(&self) -> ::windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(StockMapViewIterator::<K, V> {
                _owner: self.cast()?,
                current: std::sync::RwLock::new(self.map.iter()),
            }
            .into())
        }
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Lookup(&self, key: &K::Default) -> ::windows_core::Result<V> {
        let value = self
            .map
            .get(key)
            .ok_or_else(|| ::windows_core::Error::from(::windows_core::imp::E_BOUNDS))?;
        V::from_default(value)
    }
    fn Size(&self) -> ::windows_core::Result<u32> {
        Ok(self.map.len() as u32)
    }
    fn HasKey(&self, key: &K::Default) -> ::windows_core::Result<bool> {
        Ok(self.map.contains_key(key))
    }
    fn Split(
        &self,
        first: &mut std::option::Option<IMapView<K, V>>,
        second: &mut std::option::Option<IMapView<K, V>>,
    ) -> ::windows_core::Result<()> {
        *first = None;
        *second = None;
        Ok(())
    }
}

#[::windows_implement::implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<'a, K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    _owner: IIterable<IKeyValuePair<K, V>>,
    current: ::std::sync::RwLock<std::collections::btree_map::Iter<'a, K::Default, V::Default>>,
}

impl<'a, K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator<'a, K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Current(&self) -> ::windows_core::Result<IKeyValuePair<K, V>> {
        let mut current = self.current.read().unwrap().clone().peekable();

        if let Some((key, value)) = current.peek() {
            Ok(StockKeyValuePair {
                key: (*key).clone(),
                value: (*value).clone(),
            }
            .into())
        } else {
            Err(::windows_core::Error::from(::windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let mut current = self.current.read().unwrap().clone().peekable();

        Ok(current.peek().is_some())
    }

    fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let mut current = self.current.write().unwrap();

        current.next();
        Ok(current.clone().peekable().peek().is_some())
    }

    fn GetMany(&self, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> ::windows_core::Result<u32> {
        let mut current = self.current.write().unwrap();
        let mut actual = 0;

        for pair in pairs {
            if let Some((key, value)) = current.next() {
                *pair = Some(
                    StockKeyValuePair {
                        key: (*key).clone(),
                        value: (*value).clone(),
                    }
                    .into(),
                );
                actual += 1;
            } else {
                break;
            }
        }

        Ok(actual)
    }
}

#[::windows_implement::implement(IKeyValuePair<K, V>)]
struct StockKeyValuePair<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    key: K::Default,
    value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Key(&self) -> ::windows_core::Result<K> {
        K::from_default(&self.key)
    }
    fn Value(&self) -> ::windows_core::Result<V> {
        V::from_default(&self.value)
    }
}

impl<K, V> ::core::convert::TryFrom<std::collections::BTreeMap<K::Default, V::Default>>
    for IMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    type Error = ::windows_core::Error;
    fn try_from(
        map: std::collections::BTreeMap<K::Default, V::Default>,
    ) -> ::windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(StockMapView { map }.into())
    }
}
