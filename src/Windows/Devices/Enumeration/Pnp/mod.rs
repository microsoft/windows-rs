#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObject(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPnpObject {
    type Vtable = IPnpObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c66258_733b_4a8f_93a3_db078ac870c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PnpObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updateinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPnpObjectStatics {
    type Vtable = IPnpObjectStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c32a3d_d168_4660_bbf3_a733b14b6e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: PnpObjectType, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectUpdate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f59e812_001e_4844_bcc6_432886856a17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectUpdate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PnpObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83c95ca8_4772_4a7a_aca8_e48c42a89c44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::DeviceWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PnpObject(pub ::windows::core::IInspectable);
impl PnpObject {
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Type(&self) -> ::windows::core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__: PnpObjectType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PnpObjectType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Update<'a, Param0: ::windows::core::IntoParam<'a, PnpObjectUpdate>>(&self, updateinfo: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), updateinfo.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(r#type: PnpObjectType, id: Param1, requestedproperties: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObject>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, id.into_param().abi(), requestedproperties.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObject>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(r#type: PnpObjectType, requestedproperties: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilter<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(r#type: PnpObjectType, requestedproperties: Param1, aqsfilter: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), aqsfilter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn CreateWatcher<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(r#type: PnpObjectType, requestedproperties: Param1) -> ::windows::core::Result<PnpObjectWatcher> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), &mut result__).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn CreateWatcherAqsFilter<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(r#type: PnpObjectType, requestedproperties: Param1, aqsfilter: Param2) -> ::windows::core::Result<PnpObjectWatcher> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), aqsfilter.into_param().abi(), &mut result__).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    pub fn IPnpObjectStatics<R, F: FnOnce(&IPnpObjectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PnpObject, IPnpObjectStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})");
}
unsafe impl ::windows::core::Interface for PnpObject {
    type Vtable = IPnpObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c66258_733b_4a8f_93a3_db078ac870c1);
}
impl ::windows::core::RuntimeName for PnpObject {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObject";
}
impl ::core::convert::From<PnpObject> for ::windows::core::IUnknown {
    fn from(value: PnpObject) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PnpObject> for ::windows::core::IUnknown {
    fn from(value: &PnpObject) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PnpObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PnpObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PnpObject> for ::windows::core::IInspectable {
    fn from(value: PnpObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PnpObject> for ::windows::core::IInspectable {
    fn from(value: &PnpObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PnpObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PnpObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PnpObject {}
unsafe impl ::core::marker::Sync for PnpObject {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PnpObjectCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PnpObjectCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<PnpObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<PnpObject>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, PnpObject>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PnpObject as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<PnpObject>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<PnpObject>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<PnpObject>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PnpObjectCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PnpObjectCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<PnpObject>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<PnpObject> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PnpObjectCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PnpObjectCollection> for ::windows::core::IUnknown {
    fn from(value: PnpObjectCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PnpObjectCollection> for ::windows::core::IUnknown {
    fn from(value: &PnpObjectCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PnpObjectCollection> for ::windows::core::IInspectable {
    fn from(value: PnpObjectCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PnpObjectCollection> for ::windows::core::IInspectable {
    fn from(value: &PnpObjectCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    fn from(value: PnpObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    fn from(value: &PnpObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> for PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> for &PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> for PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> for &PnpObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<PnpObject>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PnpObjectCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PnpObjectCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PnpObjectType(pub i32);
impl PnpObjectType {
    pub const Unknown: PnpObjectType = PnpObjectType(0i32);
    pub const DeviceInterface: PnpObjectType = PnpObjectType(1i32);
    pub const DeviceContainer: PnpObjectType = PnpObjectType(2i32);
    pub const Device: PnpObjectType = PnpObjectType(3i32);
    pub const DeviceInterfaceClass: PnpObjectType = PnpObjectType(4i32);
    pub const AssociationEndpoint: PnpObjectType = PnpObjectType(5i32);
    pub const AssociationEndpointContainer: PnpObjectType = PnpObjectType(6i32);
    pub const AssociationEndpointService: PnpObjectType = PnpObjectType(7i32);
    pub const DevicePanel: PnpObjectType = PnpObjectType(8i32);
}
impl ::core::convert::From<i32> for PnpObjectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PnpObjectType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PnpObjectType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Pnp.PnpObjectType;i4)");
}
impl ::windows::core::DefaultType for PnpObjectType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PnpObjectUpdate(pub ::windows::core::IInspectable);
impl PnpObjectUpdate {
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Type(&self) -> ::windows::core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__: PnpObjectType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PnpObjectType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObjectUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectUpdate;{6f59e812-001e-4844-bcc6-432886856a17})");
}
unsafe impl ::windows::core::Interface for PnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f59e812_001e_4844_bcc6_432886856a17);
}
impl ::windows::core::RuntimeName for PnpObjectUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectUpdate";
}
impl ::core::convert::From<PnpObjectUpdate> for ::windows::core::IUnknown {
    fn from(value: PnpObjectUpdate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PnpObjectUpdate> for ::windows::core::IUnknown {
    fn from(value: &PnpObjectUpdate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PnpObjectUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PnpObjectUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PnpObjectUpdate> for ::windows::core::IInspectable {
    fn from(value: PnpObjectUpdate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PnpObjectUpdate> for ::windows::core::IInspectable {
    fn from(value: &PnpObjectUpdate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PnpObjectUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PnpObjectUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PnpObjectUpdate {}
unsafe impl ::core::marker::Sync for PnpObjectUpdate {}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PnpObjectWatcher(pub ::windows::core::IInspectable);
impl PnpObjectWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Status(&self) -> ::windows::core::Result<super::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: super::DeviceWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PnpObjectWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectWatcher;{83c95ca8-4772-4a7a-aca8-e48c42a89c44})");
}
unsafe impl ::windows::core::Interface for PnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83c95ca8_4772_4a7a_aca8_e48c42a89c44);
}
impl ::windows::core::RuntimeName for PnpObjectWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectWatcher";
}
impl ::core::convert::From<PnpObjectWatcher> for ::windows::core::IUnknown {
    fn from(value: PnpObjectWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PnpObjectWatcher> for ::windows::core::IUnknown {
    fn from(value: &PnpObjectWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PnpObjectWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PnpObjectWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PnpObjectWatcher> for ::windows::core::IInspectable {
    fn from(value: PnpObjectWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PnpObjectWatcher> for ::windows::core::IInspectable {
    fn from(value: &PnpObjectWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PnpObjectWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PnpObjectWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PnpObjectWatcher {}
unsafe impl ::core::marker::Sync for PnpObjectWatcher {}
