#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObject(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPnpObject {
    type Vtable = IPnpObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2512806488, 29499, 19087, [147, 163, 219, 7, 138, 200, 112, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PnpObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updateinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPnpObjectStatics {
    type Vtable = IPnpObjectStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3015911997, 53608, 18016, [187, 243, 167, 51, 177, 75, 110, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: PnpObjectType, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, requestedproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::runtime::RawPtr, aqsfilter: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: PnpObjectType, requestedproperties: ::windows::runtime::RawPtr, aqsfilter: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectUpdate(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868163090, 30, 18500, [188, 198, 67, 40, 134, 133, 106, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectUpdate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PnpObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPnpObjectWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2211011752, 18290, 19066, [172, 168, 228, 140, 66, 168, 156, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::DeviceWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PnpObject(::windows::runtime::IInspectable);
impl PnpObject {
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__: PnpObjectType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PnpObjectType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, PnpObjectUpdate>>(&self, updateinfo: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), updateinfo.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(r#type: PnpObjectType, id: Param1, requestedproperties: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<PnpObject>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type, id.into_param().abi(), requestedproperties.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObject>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(r#type: PnpObjectType, requestedproperties: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilter<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(r#type: PnpObjectType, requestedproperties: Param1, aqsfilter: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), aqsfilter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn CreateWatcher<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(r#type: PnpObjectType, requestedproperties: Param1) -> ::windows::runtime::Result<PnpObjectWatcher> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), &mut result__).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn CreateWatcherAqsFilter<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(r#type: PnpObjectType, requestedproperties: Param1, aqsfilter: Param2) -> ::windows::runtime::Result<PnpObjectWatcher> {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), r#type, requestedproperties.into_param().abi(), aqsfilter.into_param().abi(), &mut result__).from_abi::<PnpObjectWatcher>(result__)
        })
    }
    pub fn IPnpObjectStatics<R, F: FnOnce(&IPnpObjectStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PnpObject, IPnpObjectStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PnpObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})");
}
unsafe impl ::windows::runtime::Interface for PnpObject {
    type Vtable = IPnpObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2512806488, 29499, 19087, [147, 163, 219, 7, 138, 200, 112, 193]);
}
impl ::windows::runtime::RuntimeName for PnpObject {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObject";
}
impl ::std::convert::From<PnpObject> for ::windows::runtime::IUnknown {
    fn from(value: PnpObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PnpObject> for ::windows::runtime::IUnknown {
    fn from(value: &PnpObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PnpObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PnpObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PnpObject> for ::windows::runtime::IInspectable {
    fn from(value: PnpObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PnpObject> for ::windows::runtime::IInspectable {
    fn from(value: &PnpObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PnpObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PnpObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PnpObject {}
unsafe impl ::std::marker::Sync for PnpObject {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PnpObjectCollection(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PnpObjectCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<PnpObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<PnpObject>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, PnpObject>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<PnpObject as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<PnpObject>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<PnpObject>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<PnpObject>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PnpObjectCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.Pnp.PnpObject;{95c66258-733b-4a8f-93a3-db078ac870c1})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PnpObjectCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<PnpObject>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<PnpObject> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PnpObjectCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PnpObjectCollection> for ::windows::runtime::IUnknown {
    fn from(value: PnpObjectCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PnpObjectCollection> for ::windows::runtime::IUnknown {
    fn from(value: &PnpObjectCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PnpObjectCollection> for ::windows::runtime::IInspectable {
    fn from(value: PnpObjectCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PnpObjectCollection> for ::windows::runtime::IInspectable {
    fn from(value: &PnpObjectCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    fn from(value: PnpObjectCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PnpObjectCollection> for super::super::super::Foundation::Collections::IVectorView<PnpObject> {
    fn from(value: &PnpObjectCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> for PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IVectorView<PnpObject>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> for &PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<PnpObject>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IVectorView<PnpObject>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PnpObjectCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&PnpObjectCollection> for super::super::super::Foundation::Collections::IIterable<PnpObject> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PnpObjectCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> for PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> for &PnpObjectCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<PnpObject>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<PnpObject>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Send for PnpObjectCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Sync for PnpObjectCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PnpObjectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PnpObjectType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PnpObjectType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Pnp.PnpObjectType;i4)");
}
impl ::windows::runtime::DefaultType for PnpObjectType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PnpObjectUpdate(::windows::runtime::IInspectable);
impl PnpObjectUpdate {
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__: PnpObjectType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PnpObjectType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PnpObjectUpdate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectUpdate;{6f59e812-001e-4844-bcc6-432886856a17})");
}
unsafe impl ::windows::runtime::Interface for PnpObjectUpdate {
    type Vtable = IPnpObjectUpdate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1868163090, 30, 18500, [188, 198, 67, 40, 134, 133, 106, 23]);
}
impl ::windows::runtime::RuntimeName for PnpObjectUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectUpdate";
}
impl ::std::convert::From<PnpObjectUpdate> for ::windows::runtime::IUnknown {
    fn from(value: PnpObjectUpdate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PnpObjectUpdate> for ::windows::runtime::IUnknown {
    fn from(value: &PnpObjectUpdate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PnpObjectUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PnpObjectUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PnpObjectUpdate> for ::windows::runtime::IInspectable {
    fn from(value: PnpObjectUpdate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PnpObjectUpdate> for ::windows::runtime::IInspectable {
    fn from(value: &PnpObjectUpdate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PnpObjectUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PnpObjectUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PnpObjectUpdate {}
unsafe impl ::std::marker::Sync for PnpObjectUpdate {}
#[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PnpObjectWatcher(::windows::runtime::IInspectable);
impl PnpObjectWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration_Pnp`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: super::DeviceWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration_Pnp`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PnpObjectWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.Pnp.PnpObjectWatcher;{83c95ca8-4772-4a7a-aca8-e48c42a89c44})");
}
unsafe impl ::windows::runtime::Interface for PnpObjectWatcher {
    type Vtable = IPnpObjectWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2211011752, 18290, 19066, [172, 168, 228, 140, 66, 168, 156, 68]);
}
impl ::windows::runtime::RuntimeName for PnpObjectWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectWatcher";
}
impl ::std::convert::From<PnpObjectWatcher> for ::windows::runtime::IUnknown {
    fn from(value: PnpObjectWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PnpObjectWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &PnpObjectWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PnpObjectWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PnpObjectWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PnpObjectWatcher> for ::windows::runtime::IInspectable {
    fn from(value: PnpObjectWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PnpObjectWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &PnpObjectWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PnpObjectWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PnpObjectWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PnpObjectWatcher {}
unsafe impl ::std::marker::Sync for PnpObjectWatcher {}
