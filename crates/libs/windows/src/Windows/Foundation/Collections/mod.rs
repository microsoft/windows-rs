windows_core::imp::define_interface!(IPropertySet, IPropertySet_Vtbl, 0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
impl windows_core::RuntimeType for IPropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPropertySet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( IPropertySet , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::IInspectable > , windows_collections:: IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl IPropertySet {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows_collections::MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl IntoIterator for IPropertySet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &IPropertySet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl windows_core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
pub trait IPropertySet_Impl: windows_collections::IIterable_Impl<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>> + windows_collections::IMap_Impl<windows_core::HSTRING, windows_core::IInspectable> + windows_collections::IObservableMap_Impl<windows_core::HSTRING, windows_core::IInspectable> {}
impl IPropertySet_Vtbl {
    pub const fn new<Identity: IPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertySet, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySet as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySet_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertySet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PropertySet, windows_core::IUnknown, windows_core::IInspectable, IPropertySet);
windows_core::imp::required_hierarchy ! ( PropertySet , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::IInspectable > , windows_collections:: IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl PropertySet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PropertySet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows_collections::MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
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
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PropertySet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy ! ( StringMap , windows_core::IUnknown , windows_core::IInspectable , windows_collections:: IMap < windows_core::HSTRING , windows_core::HSTRING > );
windows_core::imp::required_hierarchy ! ( StringMap , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > > , windows_collections:: IObservableMap < windows_core::HSTRING , windows_core::HSTRING > );
impl StringMap {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StringMap, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows_collections::MapChangedEventHandler<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for StringMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>();
}
unsafe impl windows_core::Interface for StringMap {
    type Vtable = <windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StringMap {
    const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
}
unsafe impl Send for StringMap {}
unsafe impl Sync for StringMap {}
impl IntoIterator for StringMap {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &StringMap {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueSet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValueSet, windows_core::IUnknown, windows_core::IInspectable, IPropertySet);
windows_core::imp::required_hierarchy ! ( ValueSet , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::IInspectable > , windows_collections:: IObservableMap < windows_core::HSTRING , windows_core::IInspectable > );
impl ValueSet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValueSet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows_collections::MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
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
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &ValueSet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
