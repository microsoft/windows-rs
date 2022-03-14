#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct BasicProperties(::windows::core::IUnknown);
impl BasicProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateModified(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateModified)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BasicProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BasicProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BasicProperties {}
impl ::core::fmt::Debug for BasicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BasicProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BasicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.BasicProperties;{d05d55db-785e-4a66-be02-9beec58aea81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BasicProperties {
    type Vtable = IBasicProperties_Vtbl;
    const IID: ::windows::core::GUID = <IBasicProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BasicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.BasicProperties";
}
impl ::core::convert::From<BasicProperties> for ::windows::core::IUnknown {
    fn from(value: BasicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicProperties> for ::windows::core::IUnknown {
    fn from(value: &BasicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BasicProperties> for ::windows::core::IInspectable {
    fn from(value: BasicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicProperties> for ::windows::core::IInspectable {
    fn from(value: &BasicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: BasicProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &BasicProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &BasicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct DocumentProperties(::windows::core::IUnknown);
impl DocumentProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Author(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for DocumentProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DocumentProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DocumentProperties {}
impl ::core::fmt::Debug for DocumentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DocumentProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DocumentProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.DocumentProperties;{7eab19bc-1821-4923-b4a9-0aea404d0070})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DocumentProperties {
    type Vtable = IDocumentProperties_Vtbl;
    const IID: ::windows::core::GUID = <IDocumentProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DocumentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.DocumentProperties";
}
impl ::core::convert::From<DocumentProperties> for ::windows::core::IUnknown {
    fn from(value: DocumentProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DocumentProperties> for ::windows::core::IUnknown {
    fn from(value: &DocumentProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DocumentProperties> for ::windows::core::IInspectable {
    fn from(value: DocumentProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DocumentProperties> for ::windows::core::IInspectable {
    fn from(value: &DocumentProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: DocumentProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &DocumentProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &DocumentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
pub struct GeotagHelper {}
impl GeotagHelper {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetGeotagAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGeotagAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn SetGeotagFromGeolocatorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Devices::Geolocation::Geolocator>>(file: Param0, geolocator: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetGeotagFromGeolocatorAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), geolocator.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn SetGeotagAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Devices::Geolocation::Geopoint>>(file: Param0, geopoint: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetGeotagAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), geopoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeotagHelperStatics<R, F: FnOnce(&IGeotagHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeotagHelper, IGeotagHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GeotagHelper {
    const NAME: &'static str = "Windows.Storage.FileProperties.GeotagHelper";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBasicProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBasicProperties {
    type Vtable = IBasicProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd05d55db_785e_4a66_be02_9beec58aea81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateModified: usize,
    #[cfg(feature = "Foundation")]
    pub ItemDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDocumentProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDocumentProperties {
    type Vtable = IDocumentProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eab19bc_1821_4923_b4a9_0aea404d0070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocumentProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Author: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeotagHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeotagHelperStatics {
    type Vtable = IGeotagHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41493244_2524_4655_86a6_ed16f5fc716b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeotagHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetGeotagAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetGeotagAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub SetGeotagFromGeolocatorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, geolocator: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    SetGeotagFromGeolocatorAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub SetGeotagAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, geopoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    SetGeotagAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageProperties {
    type Vtable = IImageProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x523c9424_fcff_4275_afee_ecdb9ab47973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    #[cfg(feature = "Foundation")]
    pub DateTaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateTaken: usize,
    #[cfg(feature = "Foundation")]
    pub SetDateTaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDateTaken: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Latitude: usize,
    #[cfg(feature = "Foundation")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Longitude: usize,
    pub CameraManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCameraManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CameraModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCameraModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoOrientation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PeopleNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PeopleNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMusicProperties {
    type Vtable = IMusicProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc8aab62_66ec_419a_bc5d_ca65a4cb46da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAlbum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genre: usize,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Composers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Composers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Conductors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Conductors: usize,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Producers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Producers: usize,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Writers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Writers: usize,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemContentProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05294bad_bc38_48bf_85d7_770e0e2ae0ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemContentProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetMusicPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMusicPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetVideoPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetImagePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetImagePropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDocumentPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDocumentPropertiesAsync: usize,
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct IStorageItemExtraProperties(::windows::core::IUnknown);
impl IStorageItemExtraProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItemExtraProperties> for ::windows::core::IUnknown {
    fn from(value: IStorageItemExtraProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemExtraProperties> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemExtraProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemExtraProperties> for ::windows::core::IInspectable {
    fn from(value: IStorageItemExtraProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemExtraProperties> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemExtraProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemExtraProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemExtraProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemExtraProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemExtraProperties {}
impl ::core::fmt::Debug for IStorageItemExtraProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemExtraProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemExtraProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c54361b2-54cd-432b-bdbc-4b19c4b470d7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemExtraProperties {
    type Vtable = IStorageItemExtraProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54361b2_54cd_432b_bdbc_4b19c4b470d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemExtraProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SavePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestosave: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavePropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SavePropertiesAsyncOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SavePropertiesAsyncOverloadDefault: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThumbnailProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IThumbnailProperties {
    type Vtable = IThumbnailProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693dd42f_dbe7_49b5_b3b3_2893ac5d3423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OriginalWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OriginalHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReturnedSmallerCachedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ThumbnailType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoProperties {
    type Vtable = IVideoProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x719ae507_68de_4db8_97de_49998c059f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Latitude: usize,
    #[cfg(feature = "Foundation")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Longitude: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Producers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Producers: usize,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Writers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Writers: usize,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Directors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Directors: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoOrientation) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct ImageProperties(::windows::core::IUnknown);
impl ImageProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRating)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateTaken(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateTaken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDateTaken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDateTaken)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Latitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Longitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraManufacturer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetCameraManufacturer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCameraManufacturer)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraModel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetCameraModel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCameraModel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<PhotoOrientation> {
        let this = self;
        unsafe {
            let mut result__: PhotoOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoOrientation>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PeopleNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeopleNames)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageProperties {}
impl ::core::fmt::Debug for ImageProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.ImageProperties;{523c9424-fcff-4275-afee-ecdb9ab47973})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ImageProperties {
    type Vtable = IImageProperties_Vtbl;
    const IID: ::windows::core::GUID = <IImageProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.ImageProperties";
}
impl ::core::convert::From<ImageProperties> for ::windows::core::IUnknown {
    fn from(value: ImageProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageProperties> for ::windows::core::IUnknown {
    fn from(value: &ImageProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageProperties> for ::windows::core::IInspectable {
    fn from(value: ImageProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageProperties> for ::windows::core::IInspectable {
    fn from(value: &ImageProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &ImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct MusicProperties(::windows::core::IUnknown);
impl MusicProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Album)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetAlbum<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlbum)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Artist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetArtist<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetArtist)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genre(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Genre)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn TrackNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrackNumber)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRating)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumArtist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetAlbumArtist<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlbumArtist)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Composers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Composers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Conductors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Conductors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Producers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Publisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetPublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPublisher)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Writers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Year(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Year)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetYear(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYear)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for MusicProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MusicProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MusicProperties {}
impl ::core::fmt::Debug for MusicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MusicProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MusicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.MusicProperties;{bc8aab62-66ec-419a-bc5d-ca65a4cb46da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MusicProperties {
    type Vtable = IMusicProperties_Vtbl;
    const IID: ::windows::core::GUID = <IMusicProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MusicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.MusicProperties";
}
impl ::core::convert::From<MusicProperties> for ::windows::core::IUnknown {
    fn from(value: MusicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicProperties> for ::windows::core::IUnknown {
    fn from(value: &MusicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MusicProperties> for ::windows::core::IInspectable {
    fn from(value: MusicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicProperties> for ::windows::core::IInspectable {
    fn from(value: &MusicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: MusicProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &MusicProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &MusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoOrientation(pub i32);
impl PhotoOrientation {
    pub const Unspecified: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const FlipHorizontal: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
    pub const FlipVertical: Self = Self(4i32);
    pub const Transpose: Self = Self(5i32);
    pub const Rotate270: Self = Self(6i32);
    pub const Transverse: Self = Self(7i32);
    pub const Rotate90: Self = Self(8i32);
}
impl ::core::marker::Copy for PhotoOrientation {}
impl ::core::clone::Clone for PhotoOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PhotoOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PropertyPrefetchOptions(pub u32);
impl PropertyPrefetchOptions {
    pub const None: Self = Self(0u32);
    pub const MusicProperties: Self = Self(1u32);
    pub const VideoProperties: Self = Self(2u32);
    pub const ImageProperties: Self = Self(4u32);
    pub const DocumentProperties: Self = Self(8u32);
    pub const BasicProperties: Self = Self(16u32);
}
impl ::core::marker::Copy for PropertyPrefetchOptions {}
impl ::core::clone::Clone for PropertyPrefetchOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyPrefetchOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyPrefetchOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyPrefetchOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyPrefetchOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PropertyPrefetchOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PropertyPrefetchOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PropertyPrefetchOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PropertyPrefetchOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PropertyPrefetchOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyPrefetchOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PropertyPrefetchOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct StorageItemContentProperties(::windows::core::IUnknown);
impl StorageItemContentProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMusicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MusicProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMusicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MusicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVideoPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VideoProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVideoPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VideoProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetImagePropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetImagePropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDocumentPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DocumentProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDocumentPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DocumentProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageItemContentProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageItemContentProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemContentProperties {}
impl ::core::fmt::Debug for StorageItemContentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemContentProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageItemContentProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemContentProperties;{05294bad-bc38-48bf-85d7-770e0e2ae0ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemContentProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageItemContentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemContentProperties";
}
impl ::core::convert::From<StorageItemContentProperties> for ::windows::core::IUnknown {
    fn from(value: StorageItemContentProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemContentProperties> for ::windows::core::IUnknown {
    fn from(value: &StorageItemContentProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageItemContentProperties> for ::windows::core::IInspectable {
    fn from(value: StorageItemContentProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemContentProperties> for ::windows::core::IInspectable {
    fn from(value: &StorageItemContentProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemContentProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemContentProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &StorageItemContentProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct StorageItemThumbnail(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl StorageItemThumbnail {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlushAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputStreamAt)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOutputStreamAt)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloneStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRead)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanWrite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn OriginalWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn OriginalHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn ReturnedSmallerCachedSize(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReturnedSmallerCachedSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<ThumbnailType> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__: ThumbnailType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ThumbnailType>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for StorageItemThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for StorageItemThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for StorageItemThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for StorageItemThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StorageItemThumbnail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StorageItemThumbnail {
    type Vtable = super::Streams::IRandomAccessStreamWithContentType_Vtbl;
    const IID: ::windows::core::GUID = <super::Streams::IRandomAccessStreamWithContentType as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for StorageItemThumbnail {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemThumbnail";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StorageItemThumbnail> for ::windows::core::IUnknown {
    fn from(value: StorageItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StorageItemThumbnail> for ::windows::core::IUnknown {
    fn from(value: &StorageItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StorageItemThumbnail> for ::windows::core::IInspectable {
    fn from(value: StorageItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StorageItemThumbnail> for ::windows::core::IInspectable {
    fn from(value: &StorageItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IContentTypeProvider> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IContentTypeProvider> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IInputStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IInputStream> {
        ::core::convert::TryInto::<super::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IOutputStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IOutputStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStream> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStream> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStreamWithContentType> for StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStreamWithContentType> for &StorageItemThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<super::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ThumbnailMode(pub i32);
impl ThumbnailMode {
    pub const PicturesView: Self = Self(0i32);
    pub const VideosView: Self = Self(1i32);
    pub const MusicView: Self = Self(2i32);
    pub const DocumentsView: Self = Self(3i32);
    pub const ListView: Self = Self(4i32);
    pub const SingleItem: Self = Self(5i32);
}
impl ::core::marker::Copy for ThumbnailMode {}
impl ::core::clone::Clone for ThumbnailMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: Self = Self(0u32);
    pub const ReturnOnlyIfCached: Self = Self(1u32);
    pub const ResizeThumbnail: Self = Self(2u32);
    pub const UseCurrentScale: Self = Self(4u32);
}
impl ::core::marker::Copy for ThumbnailOptions {}
impl ::core::clone::Clone for ThumbnailOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ThumbnailOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ThumbnailOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ThumbnailOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ThumbnailOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ThumbnailOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: Self = Self(0i32);
    pub const Icon: Self = Self(1i32);
}
impl ::core::marker::Copy for ThumbnailType {}
impl ::core::clone::Clone for ThumbnailType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(90i32);
    pub const Rotate180: Self = Self(180i32);
    pub const Rotate270: Self = Self(270i32);
}
impl ::core::marker::Copy for VideoOrientation {}
impl ::core::clone::Clone for VideoOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.VideoOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct VideoProperties(::windows::core::IUnknown);
impl VideoProperties {
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(&self, propertiestosave: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsync)(::core::mem::transmute_copy(this), propertiestosave.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRating)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Latitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Longitude)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Producers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Publisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetPublisher<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPublisher)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Writers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Year(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Year)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn SetYear(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetYear)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Directors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Directors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<VideoOrientation> {
        let this = self;
        unsafe {
            let mut result__: VideoOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoOrientation>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoProperties {}
impl ::core::fmt::Debug for VideoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.VideoProperties;{719ae507-68de-4db8-97de-49998c059f2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VideoProperties {
    type Vtable = IVideoProperties_Vtbl;
    const IID: ::windows::core::GUID = <IVideoProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.VideoProperties";
}
impl ::core::convert::From<VideoProperties> for ::windows::core::IUnknown {
    fn from(value: VideoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoProperties> for ::windows::core::IUnknown {
    fn from(value: &VideoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoProperties> for ::windows::core::IInspectable {
    fn from(value: VideoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoProperties> for ::windows::core::IInspectable {
    fn from(value: &VideoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemExtraProperties> for &VideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemExtraProperties> {
        ::core::convert::TryInto::<IStorageItemExtraProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
