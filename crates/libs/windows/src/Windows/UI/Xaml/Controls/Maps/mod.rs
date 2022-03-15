#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct CustomMapTileDataSource(::windows::core::IUnknown);
impl CustomMapTileDataSource {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BitmapRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitmapRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBitmapRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBitmapRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<CustomMapTileDataSource> {
        Self::ICustomMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CustomMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<CustomMapTileDataSource> {
        Self::ICustomMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<CustomMapTileDataSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomMapTileDataSourceFactory<R, F: FnOnce(&ICustomMapTileDataSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CustomMapTileDataSource, ICustomMapTileDataSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CustomMapTileDataSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomMapTileDataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomMapTileDataSource {}
impl ::core::fmt::Debug for CustomMapTileDataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomMapTileDataSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomMapTileDataSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.CustomMapTileDataSource;{65da384a-2db1-4be1-b155-3d0c9ecf4799})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomMapTileDataSource {
    type Vtable = ICustomMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = <ICustomMapTileDataSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.CustomMapTileDataSource";
}
impl ::core::convert::From<CustomMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: CustomMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: &CustomMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: CustomMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: &CustomMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomMapTileDataSource> for MapTileDataSource {
    fn from(value: CustomMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CustomMapTileDataSource> for MapTileDataSource {
    fn from(value: &CustomMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for &CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapTileDataSource>::into(self))
    }
}
impl ::core::convert::From<CustomMapTileDataSource> for super::super::DependencyObject {
    fn from(value: CustomMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CustomMapTileDataSource> for super::super::DependencyObject {
    fn from(value: &CustomMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &CustomMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CustomMapTileDataSource {}
unsafe impl ::core::marker::Sync for CustomMapTileDataSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct HttpMapTileDataSource(::windows::core::IUnknown);
impl HttpMapTileDataSource {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UriFormatString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetUriFormatString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUriFormatString)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AdditionalRequestHeaders(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdditionalRequestHeaders)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AllowCaching(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowCaching)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetAllowCaching(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowCaching)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UriRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UriRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUriRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUriRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<HttpMapTileDataSource> {
        Self::IHttpMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<HttpMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<HttpMapTileDataSource> {
        Self::IHttpMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<HttpMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithUriFormatString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uriformatstring: Param0) -> ::windows::core::Result<HttpMapTileDataSource> {
        Self::IHttpMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithUriFormatString)(::core::mem::transmute_copy(this), uriformatstring.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<HttpMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithUriFormatString_compose<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Compose>(uriformatstring: Param0, compose: T) -> ::windows::core::Result<HttpMapTileDataSource> {
        Self::IHttpMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithUriFormatString)(::core::mem::transmute_copy(this), uriformatstring.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<HttpMapTileDataSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHttpMapTileDataSourceFactory<R, F: FnOnce(&IHttpMapTileDataSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMapTileDataSource, IHttpMapTileDataSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMapTileDataSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMapTileDataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMapTileDataSource {}
impl ::core::fmt::Debug for HttpMapTileDataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMapTileDataSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpMapTileDataSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.HttpMapTileDataSource;{9d03cb5c-fd79-4795-87be-7e54ca0b37d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HttpMapTileDataSource {
    type Vtable = IHttpMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = <IHttpMapTileDataSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.HttpMapTileDataSource";
}
impl ::core::convert::From<HttpMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: HttpMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: &HttpMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: HttpMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: &HttpMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMapTileDataSource> for MapTileDataSource {
    fn from(value: HttpMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HttpMapTileDataSource> for MapTileDataSource {
    fn from(value: &HttpMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for &HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapTileDataSource>::into(self))
    }
}
impl ::core::convert::From<HttpMapTileDataSource> for super::super::DependencyObject {
    fn from(value: HttpMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HttpMapTileDataSource> for super::super::DependencyObject {
    fn from(value: &HttpMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &HttpMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for HttpMapTileDataSource {}
unsafe impl ::core::marker::Sync for HttpMapTileDataSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomMapTileDataSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomMapTileDataSource {
    type Vtable = ICustomMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65da384a_2db1_4be1_b155_3d0c9ecf4799);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomMapTileDataSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitmapRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBitmapRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomMapTileDataSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomMapTileDataSourceFactory {
    type Vtable = ICustomMapTileDataSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8477947_c955_4f22_9444_a1d8d744af11);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomMapTileDataSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMapTileDataSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMapTileDataSource {
    type Vtable = IHttpMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d03cb5c_fd79_4795_87be_7e54ca0b37d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMapTileDataSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalRequestHeaders: usize,
    pub AllowCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UriRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUriRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUriRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMapTileDataSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMapTileDataSourceFactory {
    type Vtable = IHttpMapTileDataSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b4b107_84dc_4291_89f8_6d0bb612a055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMapTileDataSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithUriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalMapTileDataSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalMapTileDataSource {
    type Vtable = ILocalMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616257b5_9108_4f12_8bf4_bb3c8f6274e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalMapTileDataSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UriRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUriRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUriRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalMapTileDataSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalMapTileDataSourceFactory {
    type Vtable = ILocalMapTileDataSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5cfe9fc_72ac_4839_8a0d_011f24693c79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalMapTileDataSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithUriFormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriformatstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapActualCameraChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapActualCameraChangedEventArgs {
    type Vtable = IMapActualCameraChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaa080da_b7f4_422c_a618_bbaa7c1d814c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapActualCameraChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapActualCameraChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapActualCameraChangedEventArgs2 {
    type Vtable = IMapActualCameraChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba4c7e5_10dc_455a_9d04_1d72fb6d9b93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapActualCameraChangedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapActualCameraChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapActualCameraChangingEventArgs {
    type Vtable = IMapActualCameraChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b0dbed6_93f7_4682_8de5_a47a1cc7a945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapActualCameraChangingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapActualCameraChangingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapActualCameraChangingEventArgs2 {
    type Vtable = IMapActualCameraChangingEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2867897_40ac_4e8a_a927_510f3846a47e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapActualCameraChangingEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapBillboard(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapBillboard {
    type Vtable = IMapBillboard_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1694259d_0ae2_4f42_a02e_292ca835d39d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapBillboard_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    #[cfg(feature = "Foundation")]
    pub NormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub CollisionBehaviorDesired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT,
    pub SetCollisionBehaviorDesired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT,
    pub ReferenceCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapBillboardFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapBillboardFactory {
    type Vtable = IMapBillboardFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe45a4c5_8f09_4b86_ae28_783740eb8b31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapBillboardFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstanceFromCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapBillboardStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapBillboardStatics {
    type Vtable = IMapBillboardStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdf839fe_e1f7_4fb0_8887_7da68c647333);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapBillboardStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CollisionBehaviorDesiredProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapCamera(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapCamera {
    type Vtable = IMapCamera_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53a6b623_c0f8_4d8b_ad1e_a59598ea840b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapCamera_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub FieldOfView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFieldOfView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapCameraFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapCameraFactory {
    type Vtable = IMapCameraFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3b0f16_83af_4ace_8e71_10ad9f1e9e7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapCameraFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationAndHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationAndHeading: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationHeadingAndPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationHeadingAndPitch: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithLocationHeadingPitchRollAndFieldOfView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithLocationHeadingPitchRollAndFieldOfView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapContextRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapContextRequestedEventArgs {
    type Vtable = IMapContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdd1b423_c961_4df2_bb57_82ee0f0bb591);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapContextRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl {
    type Vtable = IMapControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42d0b851_5256_4747_9e6c_0d11e966141e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Center: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetCenter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ColorScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapColorScheme) -> ::windows::core::HRESULT,
    pub SetColorScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapColorScheme) -> ::windows::core::HRESULT,
    pub DesiredPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub LandmarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLandmarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub LoadingStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapLoadingStatus) -> ::windows::core::HRESULT,
    pub MapServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMapServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MaxZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub PedestrianFeaturesVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPedestrianFeaturesVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Style: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapStyle) -> ::windows::core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapStyle) -> ::windows::core::HRESULT,
    pub TrafficFlowVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTrafficFlowVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransformOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformOrigin: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransformOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransformOrigin: usize,
    pub WatermarkMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapWatermarkMode) -> ::windows::core::HRESULT,
    pub SetWatermarkMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapWatermarkMode) -> ::windows::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Routes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Routes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TileSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TileSources: usize,
    #[cfg(feature = "Foundation")]
    pub CenterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CenterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCenterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCenterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub HeadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub LoadingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLoadingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLoadingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MapDoubleTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapDoubleTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapDoubleTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapDoubleTapped: usize,
    #[cfg(feature = "Foundation")]
    pub MapHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapHolding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapHolding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapHolding: usize,
    #[cfg(feature = "Foundation")]
    pub MapTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapTapped: usize,
    #[cfg(feature = "Foundation")]
    pub PitchChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PitchChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePitchChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePitchChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TransformOriginChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransformOriginChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransformOriginChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransformOriginChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ZoomLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZoomLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveZoomLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveZoomLevelChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMapElementsAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMapElementsAtOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetLocationFromOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetLocationFromOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetOffsetFromLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, offset: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetOffsetFromLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub IsLocationInView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, isinview: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    IsLocationInView: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewBoundsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, margin: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewBoundsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterAndZoomAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterAndZoomAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterZoomHeadingAndPitchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterZoomHeadingAndPitchAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: ::windows::core::RawPtr, zoomlevel: ::windows::core::RawPtr, heading: ::windows::core::RawPtr, desiredpitch: ::windows::core::RawPtr, animation: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl2 {
    type Vtable = IMapControl2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1fd644d_96ec_4065_b0f0_75281da3654d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BusinessLandmarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBusinessLandmarksVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TransitFeaturesVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransitFeaturesVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PanInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapPanInteractionMode) -> ::windows::core::HRESULT,
    pub SetPanInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapPanInteractionMode) -> ::windows::core::HRESULT,
    pub RotateInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT,
    pub SetRotateInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT,
    pub TiltInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT,
    pub SetTiltInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT,
    pub ZoomInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapInteractionMode) -> ::windows::core::HRESULT,
    pub SetZoomInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapInteractionMode) -> ::windows::core::HRESULT,
    pub Is3DSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStreetsideSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Scene: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScene: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ActualCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TargetCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CustomExperience: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCustomExperience: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MapElementClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub ActualCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ActualCameraChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualCameraChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualCameraChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualCameraChanging: usize,
    #[cfg(feature = "Foundation")]
    pub TargetCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CustomExperienceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CustomExperienceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCustomExperienceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCustomExperienceChanged: usize,
    pub StartContinuousRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT,
    pub StopContinuousRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartContinuousTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rateindegreespersecond: f64) -> ::windows::core::HRESULT,
    pub StopContinuousTilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartContinuousZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rateofchangepersecond: f64) -> ::windows::core::HRESULT,
    pub StopContinuousZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryRotateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRotateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRotateToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRotateToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTiltAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTiltAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTiltToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angleindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTiltToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomInAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomInAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomOutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomOutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryZoomToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomlevel: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryZoomToAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetSceneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetSceneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetSceneWithAnimationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scene: ::windows::core::RawPtr, animationkind: MapAnimationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetSceneWithAnimationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl3 {
    type Vtable = IMapControl3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x586328f8_8cdd_40ae_9338_af2a7be845e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub MapRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapRightTapped: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl4 {
    type Vtable = IMapControl4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x068f132a_1817_466d_b7ce_419b3f8e248b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BusinessLandmarksEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBusinessLandmarksEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TransitFeaturesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransitFeaturesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub GetVisibleRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: MapVisibleRegionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    GetVisibleRegion: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl5 {
    type Vtable = IMapControl5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd9b0ffd_7823_46a2_82c9_65ddac4f365f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapProjection) -> ::windows::core::HRESULT,
    pub SetMapProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapProjection) -> ::windows::core::HRESULT,
    pub StyleSheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStyleSheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ViewPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT,
    pub SetViewPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MapContextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapContextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapContextRequested: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMapElementsAtOffsetWithRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMapElementsAtOffsetWithRadius: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetLocationFromOffsetWithReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetLocationFromOffsetWithReferenceSystem: usize,
    pub StartContinuousPan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::HRESULT,
    pub StopContinuousPan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryPanAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalpixels: f64, verticalpixels: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPanAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryPanToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryPanToAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl6 {
    type Vtable = IMapControl6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0da89a2_1041_4bea_b88a_12ac9a82e0e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Layers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Layers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetLayers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetLayers: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryGetLocationFromOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryGetLocationFromOffset: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub TryGetLocationFromOffsetWithReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    TryGetLocationFromOffsetWithReferenceSystem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl7 {
    type Vtable = IMapControl7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d86e453_0c1f_4f7e_ae66_4ad0b4987857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControl8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControl8 {
    type Vtable = IMapControl8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x009e9c46_724d_53ca_9421_7a48fc731523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControl8_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanTiltDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanTiltUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanZoomIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanZoomOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlBusinessLandmarkClickEventArgs {
    type Vtable = IMapControlBusinessLandmarkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e464922_4a1a_4797_beb7_5cf7754cb867);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlBusinessLandmarkClickEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkPointerEnteredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlBusinessLandmarkPointerEnteredEventArgs {
    type Vtable = IMapControlBusinessLandmarkPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e4081a6_ea98_4f95_8caf_5b42696ff504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlBusinessLandmarkPointerEnteredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkPointerExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlBusinessLandmarkPointerExitedEventArgs {
    type Vtable = IMapControlBusinessLandmarkPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bb52caf_f24a_46d0_b463_65f719731057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlBusinessLandmarkPointerExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkRightTappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlBusinessLandmarkRightTappedEventArgs {
    type Vtable = IMapControlBusinessLandmarkRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ab8ae7_f184_4ab1_b0b0_35c8bf0654b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlBusinessLandmarkRightTappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch")))]
    LocalLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlDataHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlDataHelper {
    type Vtable = IMapControlDataHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb0f09c_14ab_486c_9de5_5a5def0205a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlDataHelper_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkClick: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeatureClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeatureClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeatureClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeatureClick: usize,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeatureRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeatureRightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeatureRightTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeatureRightTapped: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlDataHelper2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlDataHelper2 {
    type Vtable = IMapControlDataHelper2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ce429e_562f_4c21_a674_0f11decf0fb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlDataHelper2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeaturePointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeaturePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeaturePointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeaturePointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub BusinessLandmarkPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BusinessLandmarkPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBusinessLandmarkPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBusinessLandmarkPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub TransitFeaturePointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransitFeaturePointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransitFeaturePointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransitFeaturePointerExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlDataHelperFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlDataHelperFactory {
    type Vtable = IMapControlDataHelperFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b70aa8e_02ef_469c_bbaf_dc2158d4289b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlDataHelperFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlDataHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlDataHelperStatics {
    type Vtable = IMapControlDataHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a6632d6_e944_4110_83cf_314d0722e2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlDataHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMapControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rasterrendermode: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics {
    type Vtable = IMapControlStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2c61795_2147_4f0a_942a_5493a85de807);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CenterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ColorSchemeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DesiredPitchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HeadingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LandmarksVisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LoadingStatusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MapServiceTokenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PedestrianFeaturesVisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PitchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StyleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TrafficFlowVisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TransformOriginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WatermarkModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ZoomLevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MapElementsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoutesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TileSourcesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LocationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub GetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    GetLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics2 {
    type Vtable = IMapControlStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04852b93_b446_4d31_9752_1ec69a5996ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BusinessLandmarksVisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TransitFeaturesVisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PanInteractionModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RotateInteractionModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TiltInteractionModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ZoomInteractionModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Is3DSupportedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsStreetsideSupportedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SceneProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics4 {
    type Vtable = IMapControlStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe785d97_5d13_4fa1_bf1d_84061768c183);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BusinessLandmarksEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TransitFeaturesEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics5 {
    type Vtable = IMapControlStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09626f00_b7dd_4189_a7f7_830c412deea3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapProjectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StyleSheetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ViewPaddingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics6 {
    type Vtable = IMapControlStatics6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ccfdd7f_24d1_40a2_8351_b3063a8c95a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LayersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics7 {
    type Vtable = IMapControlStatics7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f1ac4d_72c2_46b2_b7ae_790260be641b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RegionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlStatics8(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlStatics8 {
    type Vtable = IMapControlStatics8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadb7a7b0_0605_592c_bf9d_d10bdc2be47b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlStatics8_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanTiltDownProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanTiltUpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanZoomInProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanZoomOutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlTransitFeatureClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlTransitFeatureClickEventArgs {
    type Vtable = IMapControlTransitFeatureClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76179969_b765_4622_b08a_3072745a4541);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlTransitFeatureClickEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlTransitFeaturePointerEnteredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlTransitFeaturePointerEnteredEventArgs {
    type Vtable = IMapControlTransitFeaturePointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73911a4e_ec4f_479e_94a1_36e081d0d897);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlTransitFeaturePointerEnteredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlTransitFeaturePointerExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlTransitFeaturePointerExitedEventArgs {
    type Vtable = IMapControlTransitFeaturePointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a11258d_448d_44e7_bc69_d13d497154e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlTransitFeaturePointerExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapControlTransitFeatureRightTappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapControlTransitFeatureRightTappedEventArgs {
    type Vtable = IMapControlTransitFeatureRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaea1cc49_a729_4eae_a59a_3ec9a125a028);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapControlTransitFeatureRightTappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapCustomExperience(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapCustomExperience {
    type Vtable = IMapCustomExperience_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64592866_14a3_4e5f_8883_8e9c500eeede);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapCustomExperience_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapCustomExperienceChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapCustomExperienceChangedEventArgs {
    type Vtable = IMapCustomExperienceChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e6fb9b_8fc1_4042_ac34_a61b38bb7514);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapCustomExperienceChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapCustomExperienceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapCustomExperienceFactory {
    type Vtable = IMapCustomExperienceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a403fb5_a1b1_4e7f_921e_3e6b8d8ebed6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapCustomExperienceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement {
    type Vtable = IMapElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd61fc4df_b245_47f2_9ac2_43c058b1c903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement2 {
    type Vtable = IMapElement2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6619f261_fba6_4964_a7ff_f1af63ab9cb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetMapTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement3 {
    type Vtable = IMapElement3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13efbc59_45ed_48b4_93ad_e3f78f8cf218);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapStyleSheetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMapStyleSheetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MapStyleSheetEntryState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMapStyleSheetEntryState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement3D(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement3D {
    type Vtable = IMapElement3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x827af8d5_3843_48e2_bd00_0f0644fbe6a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement3D_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Pitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement3DStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement3DStatics {
    type Vtable = IMapElement3DStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6128011a_450f_442a_b9d9_aa815c71907a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement3DStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HeadingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PitchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RollProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ScaleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElement4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElement4 {
    type Vtable = IMapElement4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645883b6_1fc1_4ceb_93bd_dc2c960072e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElement4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementClickEventArgs {
    type Vtable = IMapElementClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40168a11_d080_4519_99a1_3149fb8999d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementClickEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementFactory {
    type Vtable = IMapElementFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a30d007_0bd6_47a5_860b_7e7cf5f0c573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementPointerEnteredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementPointerEnteredEventArgs {
    type Vtable = IMapElementPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab85dd4e_91d7_4b31_8f0a_d390c7d3a2ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementPointerEnteredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementPointerExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementPointerExitedEventArgs {
    type Vtable = IMapElementPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1a45af9_60c9_4679_9119_20abc75d931f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementPointerExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementStatics {
    type Vtable = IMapElementStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c71cf2_bfef_4b49_8e99_41b5e3789fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementStatics2 {
    type Vtable = IMapElementStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf72f30_80fe_4f30_bcc1_fa894050f676);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapTabIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementStatics3 {
    type Vtable = IMapElementStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe11ee92f_9742_49aa_aad8_2e466bff3796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapStyleSheetEntryProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MapStyleSheetEntryStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TagProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementStatics4 {
    type Vtable = IMapElementStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4296f0b_dff8_467c_9315_6f6db93ee2ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayer {
    type Vtable = IMapElementsLayer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde79689a_01ef_46f4_ac60_7c200b552610);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetMapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetMapElements: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementClick: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub MapElementPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapElementPointerExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapElementPointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub MapContextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MapContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMapContextRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMapContextRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayerClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayerClickEventArgs {
    type Vtable = IMapElementsLayerClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ca7cf66_af1b_4c05_8c85_f74ae3d4677f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayerClickEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayerContextRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayerContextRequestedEventArgs {
    type Vtable = IMapElementsLayerContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda45d0b3_7a0e_4758_808b_3a637627eb0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayerContextRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MapElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MapElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayerPointerEnteredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayerPointerEnteredEventArgs {
    type Vtable = IMapElementsLayerPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757fc032_4694_4404_8c89_348b6b76c5e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayerPointerEnteredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayerPointerExitedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayerPointerExitedEventArgs {
    type Vtable = IMapElementsLayerPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92f3c6ad_03ed_4c39_af20_2a07ee1ccea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayerPointerExitedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub MapElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapElementsLayerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapElementsLayerStatics {
    type Vtable = IMapElementsLayerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34005727_f509_4d28_9180_911c03411d74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapElementsLayerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapElementsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapIcon {
    type Vtable = IMapIcon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2096872_23d9_4a2b_8be0_69f3a85482ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapIcon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetLocation: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedAnchorPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedAnchorPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedAnchorPoint: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapIcon2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapIcon2 {
    type Vtable = IMapIcon2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x611254b9_d8aa_4bbd_a316_badf06911d63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapIcon2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CollisionBehaviorDesired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapElementCollisionBehavior) -> ::windows::core::HRESULT,
    pub SetCollisionBehaviorDesired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapElementCollisionBehavior) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapIconStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapIconStatics {
    type Vtable = IMapIconStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbc9e56_1190_4b5d_9e56_e5b6724aa328);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapIconStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TitleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NormalizedAnchorPointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapIconStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapIconStatics2 {
    type Vtable = IMapIconStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4c306a_cf76_46ab_a12f_b603b986c696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapIconStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CollisionBehaviorDesiredProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapInputEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapInputEventArgs {
    type Vtable = IMapInputEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc503a0_a8a2_4394_92e9_2247764f2f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapInputEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapItemsControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapItemsControl {
    type Vtable = IMapItemsControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94c2c4d3_b335_42c5_b660_e6a07ec3bddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapItemsControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ItemsSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetItemsSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapItemsControlStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapItemsControlStatics {
    type Vtable = IMapItemsControlStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a859c7_789b_425c_8a0a_32385896cb4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapItemsControlStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ItemsSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLayer {
    type Vtable = IMapLayer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d0ff9c1_a14d_4f97_8f57_46715b57683a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLayer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetMapTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLayerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLayerFactory {
    type Vtable = IMapLayerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe02a2207_dee3_47c8_9825_bd029c5752f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLayerFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapLayerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapLayerStatics {
    type Vtable = IMapLayerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ca4a26b_5db9_4f0c_bdd5_b1bffdcce946);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLayerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MapTabIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapModel3D(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapModel3D {
    type Vtable = IMapModel3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8c541a1_ca27_4968_a2bf_9c20f06a0468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapModel3D_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapModel3DFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapModel3DFactory {
    type Vtable = IMapModel3DFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7f0bcc_580a_498b_939b_0119a9dadb9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapModel3DFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapModel3DStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapModel3DStatics {
    type Vtable = IMapModel3DStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4834a480_8e56_4b0f_872d_7ead103187cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapModel3DStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFrom3MFAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFrom3MFAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFrom3MFWithShadingOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, shadingoption: MapModel3DShadingOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFrom3MFWithShadingOptionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapPolygon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapPolygon {
    type Vtable = IMapPolygon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabda2285_4926_4c3a_a5f9_19df7f69db3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapPolygon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPath: usize,
    pub StrokeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetStrokeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub StrokeDashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStrokeDashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub FillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetFillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapPolygon2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapPolygon2 {
    type Vtable = IMapPolygon2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96c8a11e_636b_4018_9c2e_acc9122a01b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapPolygon2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub Paths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    Paths: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapPolygonStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapPolygonStatics {
    type Vtable = IMapPolygonStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37f573be_097b_424c_87cc_2ee042fda6d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapPolygonStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PathProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeDashedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapPolyline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapPolyline {
    type Vtable = IMapPolyline_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbad24a2_24df_4a86_8ffa_0f8f4d9ec17d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapPolyline_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPath: usize,
    pub StrokeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetStrokeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub StrokeDashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStrokeDashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapPolylineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapPolylineStatics {
    type Vtable = IMapPolylineStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61fde44b_1ddf_4303_b809_ec87f58ad065);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapPolylineStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PathProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StrokeDashedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRightTappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRightTappedEventArgs {
    type Vtable = IMapRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20943171_6fe8_40a6_ad0e_297379b575a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRightTappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteView {
    type Vtable = IMapRouteView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x740eaec5_bacc_41e1_a67e_dd6513832049);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RouteColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetRouteColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
    pub OutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub SetOutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Services_Maps")]
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Services_Maps"))]
    Route: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapRouteViewFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapRouteViewFactory {
    type Vtable = IMapRouteViewFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf083addf_0066_4628_82fe_ea78c23cec1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteViewFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Services_Maps")]
    pub CreateInstanceWithMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Services_Maps"))]
    CreateInstanceWithMapRoute: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapScene(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapScene {
    type Vtable = IMapScene_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bba10a9_50e7_482c_9789_c688b178ac24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapScene_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TargetCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TargetCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetCameraChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetCameraChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetCameraChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapSceneStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapSceneStatics {
    type Vtable = IMapSceneStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e4ad6c_86ec_44d9_9597_fb75b7deba0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapSceneStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromBoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromBoundingBoxWithHeadingAndPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromBoundingBoxWithHeadingAndPitch: usize,
    pub CreateFromCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, camera: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationWithHeadingAndPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationWithHeadingAndPitch: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationAndRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationAndRadius: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromLocationAndRadiusWithHeadingAndPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromLocationAndRadiusWithHeadingAndPitch: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub CreateFromLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    CreateFromLocations: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub CreateFromLocationsWithHeadingAndPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locations: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    CreateFromLocationsWithHeadingAndPitch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapStyleSheet(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapStyleSheet {
    type Vtable = IMapStyleSheet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae54b2bf_8991_42ed_8d58_20473deede1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapStyleSheet_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapStyleSheetEntriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapStyleSheetEntriesStatics {
    type Vtable = IMapStyleSheetEntriesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9636345_ef1a_41a4_a757_bd4f43e1e4d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapStyleSheetEntriesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Area: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Airport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Cemetery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Continent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Education: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IndigenousPeoplesReserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Island: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Medical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Military: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Nautical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Neighborhood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Runway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShoppingCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Stadium: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Vegetation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Forest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GolfCourse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Park: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PlayingField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Reserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NaturalPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Peak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VolcanicPeak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WaterPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PointOfInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Business: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FoodPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PopulatedPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capital: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AdminDistrictCapital: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CountryRegionCapital: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoadShield: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoadExit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Transit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Political: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CountryRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AdminDistrict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub District: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Structure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Building: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EducationBuilding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MedicalBuilding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TransitBuilding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Transportation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ControlledAccessHighway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HighSpeedRamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Highway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MajorRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ArterialRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ramp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UnpavedStreet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TollRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Railway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Trail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WaterRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Water: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub River: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RouteLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WalkingRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DrivingRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapStyleSheetEntryStatesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapStyleSheetEntryStatesStatics {
    type Vtable = IMapStyleSheetEntryStatesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23ac5532_866d_4bfa_b481_39bea1de3506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapStyleSheetEntryStatesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapStyleSheetStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapStyleSheetStatics {
    type Vtable = IMapStyleSheetStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabbd00ad_0a1c_4335_82f4_61d936aa197d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapStyleSheetStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Aerial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AerialWithOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoadLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoadDark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoadHighContrastLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoadHighContrastDark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Combine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesheets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Combine: usize,
    pub ParseFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, styleasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stylesheet: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTargetCameraChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTargetCameraChangedEventArgs {
    type Vtable = IMapTargetCameraChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbf00472_e953_4fa8_97d0_ea86359057cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTargetCameraChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTargetCameraChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTargetCameraChangedEventArgs2 {
    type Vtable = IMapTargetCameraChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97c0b332_f2b6_460b_8d91_ac020a2383dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTargetCameraChangedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapCameraChangeReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileBitmapRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileBitmapRequest {
    type Vtable = IMapTileBitmapRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46733fbc_d89d_472b_b5f6_d7066b0584f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileBitmapRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPixelData: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileBitmapRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileBitmapRequestDeferral {
    type Vtable = IMapTileBitmapRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe370542_a4ac_4efa_9665_0490b0cafdd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileBitmapRequestDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileBitmapRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileBitmapRequestedEventArgs {
    type Vtable = IMapTileBitmapRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x337f691d_9b02_4aa2_8b1e_cc4d91719bf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileBitmapRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub X: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileBitmapRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileBitmapRequestedEventArgs2 {
    type Vtable = IMapTileBitmapRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0261d114_246a_5296_bc85_590f53aa39c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileBitmapRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FrameIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileDataSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileDataSource {
    type Vtable = IMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03d9f5e_be1f_4c69_9969_79467a513c38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileDataSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileDataSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileDataSourceFactory {
    type Vtable = IMapTileDataSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3920fbd_e446_4648_a74d_fd2c5d557c06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileDataSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileSource {
    type Vtable = IMapTileSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88a76e4e_2fdf_4567_9255_1100519c8d62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDataSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Layer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapTileLayer) -> ::windows::core::HRESULT,
    pub SetLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapTileLayer) -> ::windows::core::HRESULT,
    pub ZoomLevelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapZoomLevelRange) -> ::windows::core::HRESULT,
    pub SetZoomLevelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapZoomLevelRange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Bounds: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetBounds: usize,
    pub AllowOverstretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowOverstretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsFadingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsFadingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsTransparencyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsTransparencyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRetryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRetryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub TilePixelSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTilePixelSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileSource2 {
    type Vtable = IMapTileSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e65ebbd_4095_5c15_99f1_1260b4e8b0a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileSource2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AnimationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapTileAnimationState) -> ::windows::core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetFrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFrameDuration: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileSourceFactory {
    type Vtable = IMapTileSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd7f811f_77fa_482b_9d34_71d31d465c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithDataSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithDataSourceAndZoomRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithDataSourceZoomRangeAndBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithDataSourceZoomRangeAndBounds: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datasource: ::windows::core::RawPtr, zoomlevelrange: MapZoomLevelRange, bounds: ::windows::core::RawPtr, tilesizeinpixels: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileSourceStatics {
    type Vtable = IMapTileSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93fcc93c_7035_4603_99b1_e659921b6093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LayerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ZoomLevelRangeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BoundsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AllowOverstretchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsFadingEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsTransparencyEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsRetryEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ZIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TilePixelSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VisibleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileSourceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileSourceStatics2 {
    type Vtable = IMapTileSourceStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cdd47e_669c_50fd_ad85_5ea5174cf59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileSourceStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AnimationStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FrameCountProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FrameDurationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileUriRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileUriRequest {
    type Vtable = IMapTileUriRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17402335_3127_45b8_87a7_99f87d4e2745);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileUriRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileUriRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileUriRequestDeferral {
    type Vtable = IMapTileUriRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc117ade0_bf3e_4c51_8faa_4b593cf68eb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileUriRequestDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileUriRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileUriRequestedEventArgs {
    type Vtable = IMapTileUriRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2147b43_1bbf_4b98_8dd3_b7834e407e0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileUriRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub X: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapTileUriRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapTileUriRequestedEventArgs2 {
    type Vtable = IMapTileUriRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2302185d_33b5_5a55_92f5_74a86a22efa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapTileUriRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FrameIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreetsideExperience(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreetsideExperience {
    type Vtable = IStreetsideExperience_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa558aec9_e30c_46c8_8116_484691675558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreetsideExperience_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddressTextVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAddressTextVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CursorVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCursorVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OverviewMapVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetOverviewMapVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StreetLabelsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStreetLabelsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ExitButtonVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetExitButtonVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ZoomButtonsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetZoomButtonsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreetsideExperienceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreetsideExperienceFactory {
    type Vtable = IStreetsideExperienceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a5bcf3c_649e_4342_9995_68a6cf5961a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreetsideExperienceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstanceWithPanorama: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithPanoramaHeadingPitchAndFieldOfView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, panorama: ::windows::core::RawPtr, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreetsidePanorama(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreetsidePanorama {
    type Vtable = IStreetsidePanorama_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fe00fd8_ad60_4664_b539_b1069f16c5af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreetsidePanorama_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreetsidePanoramaStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreetsidePanoramaStatics {
    type Vtable = IStreetsidePanoramaStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3b47f69_54b3_4ec5_b2a0_4f8204576507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreetsidePanoramaStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindNearbyWithLocationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindNearbyWithLocationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindNearbyWithLocationAndRadiusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, radiusinmeters: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindNearbyWithLocationAndRadiusAsync: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct LocalMapTileDataSource(::windows::core::IUnknown);
impl LocalMapTileDataSource {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UriFormatString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetUriFormatString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUriFormatString)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UriRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UriRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUriRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUriRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<LocalMapTileDataSource> {
        Self::ILocalMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<LocalMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<LocalMapTileDataSource> {
        Self::ILocalMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<LocalMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithUriFormatString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uriformatstring: Param0) -> ::windows::core::Result<LocalMapTileDataSource> {
        Self::ILocalMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithUriFormatString)(::core::mem::transmute_copy(this), uriformatstring.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<LocalMapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithUriFormatString_compose<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Compose>(uriformatstring: Param0, compose: T) -> ::windows::core::Result<LocalMapTileDataSource> {
        Self::ILocalMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithUriFormatString)(::core::mem::transmute_copy(this), uriformatstring.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<LocalMapTileDataSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalMapTileDataSourceFactory<R, F: FnOnce(&ILocalMapTileDataSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalMapTileDataSource, ILocalMapTileDataSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LocalMapTileDataSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalMapTileDataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalMapTileDataSource {}
impl ::core::fmt::Debug for LocalMapTileDataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalMapTileDataSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalMapTileDataSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.LocalMapTileDataSource;{616257b5-9108-4f12-8bf4-bb3c8f6274e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LocalMapTileDataSource {
    type Vtable = ILocalMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = <ILocalMapTileDataSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalMapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.LocalMapTileDataSource";
}
impl ::core::convert::From<LocalMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: LocalMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalMapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: &LocalMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: LocalMapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalMapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: &LocalMapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalMapTileDataSource> for MapTileDataSource {
    fn from(value: LocalMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LocalMapTileDataSource> for MapTileDataSource {
    fn from(value: &LocalMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapTileDataSource> for &LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, MapTileDataSource> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapTileDataSource>::into(self))
    }
}
impl ::core::convert::From<LocalMapTileDataSource> for super::super::DependencyObject {
    fn from(value: LocalMapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LocalMapTileDataSource> for super::super::DependencyObject {
    fn from(value: &LocalMapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &LocalMapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LocalMapTileDataSource {}
unsafe impl ::core::marker::Sync for LocalMapTileDataSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapActualCameraChangedEventArgs(::windows::core::IUnknown);
impl MapActualCameraChangedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapActualCameraChangedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Camera(&self) -> ::windows::core::Result<MapCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Camera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason> {
        let this = &::windows::core::Interface::cast::<IMapActualCameraChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: MapCameraChangeReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCameraChangeReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MapActualCameraChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapActualCameraChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapActualCameraChangedEventArgs {}
impl ::core::fmt::Debug for MapActualCameraChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapActualCameraChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapActualCameraChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapActualCameraChangedEventArgs;{daa080da-b7f4-422c-a618-bbaa7c1d814c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapActualCameraChangedEventArgs {
    type Vtable = IMapActualCameraChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapActualCameraChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapActualCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapActualCameraChangedEventArgs";
}
impl ::core::convert::From<MapActualCameraChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapActualCameraChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapActualCameraChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapActualCameraChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapActualCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapActualCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapActualCameraChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapActualCameraChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapActualCameraChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapActualCameraChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapActualCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapActualCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapActualCameraChangedEventArgs {}
unsafe impl ::core::marker::Sync for MapActualCameraChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapActualCameraChangingEventArgs(::windows::core::IUnknown);
impl MapActualCameraChangingEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapActualCameraChangingEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Camera(&self) -> ::windows::core::Result<MapCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Camera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason> {
        let this = &::windows::core::Interface::cast::<IMapActualCameraChangingEventArgs2>(self)?;
        unsafe {
            let mut result__: MapCameraChangeReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCameraChangeReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MapActualCameraChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapActualCameraChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapActualCameraChangingEventArgs {}
impl ::core::fmt::Debug for MapActualCameraChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapActualCameraChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapActualCameraChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapActualCameraChangingEventArgs;{6b0dbed6-93f7-4682-8de5-a47a1cc7a945})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapActualCameraChangingEventArgs {
    type Vtable = IMapActualCameraChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapActualCameraChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapActualCameraChangingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapActualCameraChangingEventArgs";
}
impl ::core::convert::From<MapActualCameraChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapActualCameraChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapActualCameraChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapActualCameraChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapActualCameraChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapActualCameraChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapActualCameraChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapActualCameraChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapActualCameraChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapActualCameraChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapActualCameraChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapActualCameraChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapActualCameraChangingEventArgs {}
unsafe impl ::core::marker::Sync for MapActualCameraChangingEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapAnimationKind(pub i32);
impl MapAnimationKind {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Linear: Self = Self(2i32);
    pub const Bow: Self = Self(3i32);
}
impl ::core::marker::Copy for MapAnimationKind {}
impl ::core::clone::Clone for MapAnimationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapAnimationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapAnimationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapAnimationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapAnimationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapAnimationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapAnimationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapBillboard(::windows::core::IUnknown);
impl MapBillboard {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NormalizedAnchorPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNormalizedAnchorPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNormalizedAnchorPoint)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Image)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetImage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior> {
        let this = self;
        unsafe {
            let mut result__: MapElementCollisionBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollisionBehaviorDesired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElementCollisionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCollisionBehaviorDesired)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ReferenceCamera(&self) -> ::windows::core::Result<MapCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReferenceCamera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceFromCamera<'a, Param0: ::windows::core::IntoParam<'a, MapCamera>>(camera: Param0) -> ::windows::core::Result<MapBillboard> {
        Self::IMapBillboardFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceFromCamera)(::core::mem::transmute_copy(this), camera.into_param().abi(), &mut result__).from_abi::<MapBillboard>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LocationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapBillboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn NormalizedAnchorPointProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapBillboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NormalizedAnchorPointProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CollisionBehaviorDesiredProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapBillboardStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollisionBehaviorDesiredProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapBillboardFactory<R, F: FnOnce(&IMapBillboardFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapBillboard, IMapBillboardFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapBillboardStatics<R, F: FnOnce(&IMapBillboardStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapBillboard, IMapBillboardStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapBillboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapBillboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapBillboard {}
impl ::core::fmt::Debug for MapBillboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapBillboard").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapBillboard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapBillboard;{1694259d-0ae2-4f42-a02e-292ca835d39d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapBillboard {
    type Vtable = IMapBillboard_Vtbl;
    const IID: ::windows::core::GUID = <IMapBillboard as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapBillboard {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapBillboard";
}
impl ::core::convert::From<MapBillboard> for ::windows::core::IUnknown {
    fn from(value: MapBillboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapBillboard> for ::windows::core::IUnknown {
    fn from(value: &MapBillboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapBillboard> for ::windows::core::IInspectable {
    fn from(value: MapBillboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapBillboard> for ::windows::core::IInspectable {
    fn from(value: &MapBillboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapBillboard> for MapElement {
    fn from(value: MapBillboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapBillboard> for MapElement {
    fn from(value: &MapBillboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for &MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapElement>::into(self))
    }
}
impl ::core::convert::From<MapBillboard> for super::super::DependencyObject {
    fn from(value: MapBillboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapBillboard> for super::super::DependencyObject {
    fn from(value: &MapBillboard) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapBillboard {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapBillboard {}
unsafe impl ::core::marker::Sync for MapBillboard {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapCamera(::windows::core::IUnknown);
impl MapCamera {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Heading(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Heading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetHeading(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeading)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Pitch(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pitch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetPitch(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPitch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Roll(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Roll)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetRoll(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoll)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FieldOfView(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FieldOfView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetFieldOfView(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFieldOfView)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0) -> ::windows::core::Result<MapCamera> {
        Self::IMapCameraFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithLocation)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<MapCamera>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithLocationAndHeading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, headingindegrees: f64) -> ::windows::core::Result<MapCamera> {
        Self::IMapCameraFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithLocationAndHeading)(::core::mem::transmute_copy(this), location.into_param().abi(), headingindegrees, &mut result__).from_abi::<MapCamera>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithLocationHeadingAndPitch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapCamera> {
        Self::IMapCameraFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithLocationHeadingAndPitch)(::core::mem::transmute_copy(this), location.into_param().abi(), headingindegrees, pitchindegrees, &mut result__).from_abi::<MapCamera>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<MapCamera> {
        Self::IMapCameraFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithLocationHeadingPitchRollAndFieldOfView)(::core::mem::transmute_copy(this), location.into_param().abi(), headingindegrees, pitchindegrees, rollindegrees, fieldofviewindegrees, &mut result__).from_abi::<MapCamera>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapCameraFactory<R, F: FnOnce(&IMapCameraFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapCamera, IMapCameraFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapCamera {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapCamera {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapCamera {}
impl ::core::fmt::Debug for MapCamera {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapCamera").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapCamera {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapCamera;{53a6b623-c0f8-4d8b-ad1e-a59598ea840b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapCamera {
    type Vtable = IMapCamera_Vtbl;
    const IID: ::windows::core::GUID = <IMapCamera as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapCamera {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapCamera";
}
impl ::core::convert::From<MapCamera> for ::windows::core::IUnknown {
    fn from(value: MapCamera) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCamera> for ::windows::core::IUnknown {
    fn from(value: &MapCamera) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapCamera> for ::windows::core::IInspectable {
    fn from(value: MapCamera) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCamera> for ::windows::core::IInspectable {
    fn from(value: &MapCamera) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapCamera> for super::super::DependencyObject {
    fn from(value: MapCamera) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapCamera> for super::super::DependencyObject {
    fn from(value: &MapCamera) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapCamera {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapCamera {}
unsafe impl ::core::marker::Sync for MapCamera {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapCameraChangeReason(pub i32);
impl MapCameraChangeReason {
    pub const System: Self = Self(0i32);
    pub const UserInteraction: Self = Self(1i32);
    pub const Programmatic: Self = Self(2i32);
}
impl ::core::marker::Copy for MapCameraChangeReason {}
impl ::core::clone::Clone for MapCameraChangeReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapCameraChangeReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapCameraChangeReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapCameraChangeReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapCameraChangeReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapCameraChangeReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapCameraChangeReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapColorScheme(pub i32);
impl MapColorScheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
impl ::core::marker::Copy for MapColorScheme {}
impl ::core::clone::Clone for MapColorScheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapColorScheme {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapColorScheme {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapColorScheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapColorScheme").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapColorScheme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapColorScheme;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapContextRequestedEventArgs(::windows::core::IUnknown);
impl MapContextRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapContextRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapContextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapContextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapContextRequestedEventArgs {}
impl ::core::fmt::Debug for MapContextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapContextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapContextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapContextRequestedEventArgs;{fdd1b423-c961-4df2-bb57-82ee0f0bb591})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapContextRequestedEventArgs {
    type Vtable = IMapContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapContextRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapContextRequestedEventArgs";
}
impl ::core::convert::From<MapContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MapContextRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControl(::windows::core::IUnknown);
impl MapControl {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Center)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetCenter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCenter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Children)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ColorScheme(&self) -> ::windows::core::Result<MapColorScheme> {
        let this = self;
        unsafe {
            let mut result__: MapColorScheme = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorScheme)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapColorScheme>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetColorScheme(&self, value: MapColorScheme) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorScheme)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DesiredPitch(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredPitch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetDesiredPitch(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredPitch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Heading(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Heading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetHeading(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeading)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LandmarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LandmarksVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLandmarksVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LoadingStatus(&self) -> ::windows::core::Result<MapLoadingStatus> {
        let this = self;
        unsafe {
            let mut result__: MapLoadingStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadingStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapLoadingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapServiceToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapServiceToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapServiceToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMapServiceToken)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MaxZoomLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxZoomLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MinZoomLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinZoomLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PedestrianFeaturesVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PedestrianFeaturesVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetPedestrianFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPedestrianFeaturesVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Pitch(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pitch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Style(&self) -> ::windows::core::Result<MapStyle> {
        let this = self;
        unsafe {
            let mut result__: MapStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Style)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStyle(&self, value: MapStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TrafficFlowVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrafficFlowVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTrafficFlowVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrafficFlowVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransformOrigin(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformOrigin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTransformOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTransformOrigin)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn WatermarkMode(&self) -> ::windows::core::Result<MapWatermarkMode> {
        let this = self;
        unsafe {
            let mut result__: MapWatermarkMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WatermarkMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapWatermarkMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetWatermarkMode(&self, value: MapWatermarkMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWatermarkMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZoomLevel(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZoomLevel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapRouteView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Routes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapRouteView>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TileSources(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapTileSource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TileSources)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapTileSource>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CenterChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CenterChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCenterChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCenterChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeadingChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadingChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadingStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLoadingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoadingStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapDoubleTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapDoubleTapped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapDoubleTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapDoubleTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapHolding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapHolding)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapHolding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapHolding)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapTapped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PitchChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PitchChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePitchChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePitchChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransformOriginChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformOriginChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransformOriginChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransformOriginChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ZoomLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveZoomLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveZoomLevelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMapElementsAtOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindMapElementsAtOffset)(::core::mem::transmute_copy(this), offset.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetLocationFromOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetLocationFromOffset)(::core::mem::transmute_copy(this), offset.into_param().abi(), location as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetOffsetFromLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, location: Param0, offset: &mut super::super::super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetOffsetFromLocation)(::core::mem::transmute_copy(this), location.into_param().abi(), offset).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn IsLocationInView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, location: Param0, isinview: &mut bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).IsLocationInView)(::core::mem::transmute_copy(this), location.into_param().abi(), isinview).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TrySetViewBoundsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<super::super::Thickness>>>(&self, bounds: Param0, margin: Param1, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetViewBoundsAsync)(::core::mem::transmute_copy(this), bounds.into_param().abi(), margin.into_param().abi(), animation, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TrySetViewWithCenterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, center: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetViewWithCenterAsync)(::core::mem::transmute_copy(this), center.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TrySetViewWithCenterAndZoomAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, center: Param0, zoomlevel: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetViewWithCenterAndZoomAsync)(::core::mem::transmute_copy(this), center.into_param().abi(), zoomlevel.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TrySetViewWithCenterZoomHeadingAndPitchAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, center: Param0, zoomlevel: Param1, heading: Param2, desiredpitch: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetViewWithCenterZoomHeadingAndPitchAsync)(::core::mem::transmute_copy(this), center.into_param().abi(), zoomlevel.into_param().abi(), heading.into_param().abi(), desiredpitch.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::IReference<f64>>>(&self, center: Param0, zoomlevel: Param1, heading: Param2, desiredpitch: Param3, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync)(::core::mem::transmute_copy(this), center.into_param().abi(), zoomlevel.into_param().abi(), heading.into_param().abi(), desiredpitch.into_param().abi(), animation, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn BusinessLandmarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarksVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetBusinessLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBusinessLandmarksVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransitFeaturesVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturesVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTransitFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTransitFeaturesVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PanInteractionMode(&self) -> ::windows::core::Result<MapPanInteractionMode> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: MapPanInteractionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PanInteractionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapPanInteractionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetPanInteractionMode(&self, value: MapPanInteractionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPanInteractionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RotateInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: MapInteractionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotateInteractionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapInteractionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetRotateInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRotateInteractionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TiltInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: MapInteractionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TiltInteractionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapInteractionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTiltInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTiltInteractionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: MapInteractionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomInteractionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapInteractionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZoomInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetZoomInteractionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Is3DSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is3DSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsStreetsideSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStreetsideSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Scene(&self) -> ::windows::core::Result<MapScene> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scene)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapScene>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetScene<'a, Param0: ::windows::core::IntoParam<'a, MapScene>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetScene)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ActualCamera(&self) -> ::windows::core::Result<MapCamera> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualCamera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TargetCamera(&self) -> ::windows::core::Result<MapCamera> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCamera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CustomExperience(&self) -> ::windows::core::Result<MapCustomExperience> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomExperience)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCustomExperience>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetCustomExperience<'a, Param0: ::windows::core::IntoParam<'a, MapCustomExperience>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomExperience)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementClick)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementClick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementPointerEntered)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementPointerEntered)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementPointerExited)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementPointerExited)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualCameraChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActualCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActualCameraChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualCameraChanging<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualCameraChanging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActualCameraChanging<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActualCameraChanging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCameraChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTargetCameraChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CustomExperienceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomExperienceChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCustomExperienceChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCustomExperienceChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StartContinuousRotate(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartContinuousRotate)(::core::mem::transmute_copy(this), rateindegreespersecond).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StopContinuousRotate(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopContinuousRotate)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StartContinuousTilt(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartContinuousTilt)(::core::mem::transmute_copy(this), rateindegreespersecond).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StopContinuousTilt(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopContinuousTilt)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StartContinuousZoom(&self, rateofchangepersecond: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartContinuousZoom)(::core::mem::transmute_copy(this), rateofchangepersecond).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StopContinuousZoom(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopContinuousZoom)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRotateAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryRotateAsync)(::core::mem::transmute_copy(this), degrees, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRotateToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryRotateToAsync)(::core::mem::transmute_copy(this), angleindegrees, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTiltAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTiltAsync)(::core::mem::transmute_copy(this), degrees, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTiltToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTiltToAsync)(::core::mem::transmute_copy(this), angleindegrees, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryZoomInAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryZoomInAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryZoomOutAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryZoomOutAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryZoomToAsync(&self, zoomlevel: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryZoomToAsync)(::core::mem::transmute_copy(this), zoomlevel, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetSceneAsync<'a, Param0: ::windows::core::IntoParam<'a, MapScene>>(&self, scene: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetSceneAsync)(::core::mem::transmute_copy(this), scene.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetSceneWithAnimationAsync<'a, Param0: ::windows::core::IntoParam<'a, MapScene>>(&self, scene: Param0, animationkind: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetSceneWithAnimationAsync)(::core::mem::transmute_copy(this), scene.into_param().abi(), animationkind, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl3>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapRightTapped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapRightTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn BusinessLandmarksEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarksEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetBusinessLandmarksEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBusinessLandmarksEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransitFeaturesEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturesEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTransitFeaturesEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTransitFeaturesEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn GetVisibleRegion(&self, region: MapVisibleRegionKind) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath> {
        let this = &::windows::core::Interface::cast::<IMapControl4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVisibleRegion)(::core::mem::transmute_copy(this), region, &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapProjection(&self) -> ::windows::core::Result<MapProjection> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: MapProjection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapProjection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapProjection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapProjection(&self, value: MapProjection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMapProjection)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StyleSheet(&self) -> ::windows::core::Result<MapStyleSheet> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSheet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStyleSheet<'a, Param0: ::windows::core::IntoParam<'a, MapStyleSheet>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStyleSheet)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ViewPadding(&self) -> ::windows::core::Result<super::super::Thickness> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: super::super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewPadding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetViewPadding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetViewPadding)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapContextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapContextRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapContextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapContextRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMapElementsAtOffsetWithRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0, radius: f64) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindMapElementsAtOffsetWithRadius)(::core::mem::transmute_copy(this), offset.into_param().abi(), radius, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetLocationFromOffsetWithReferenceSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetLocationFromOffsetWithReferenceSystem)(::core::mem::transmute_copy(this), offset.into_param().abi(), desiredreferencesystem, location as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StartContinuousPan(&self, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartContinuousPan)(::core::mem::transmute_copy(this), horizontalpixelspersecond, verticalpixelspersecond).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StopContinuousPan(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopContinuousPan)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPanAsync(&self, horizontalpixels: f64, verticalpixels: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryPanAsync)(::core::mem::transmute_copy(this), horizontalpixels, verticalpixels, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TryPanToAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, location: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IMapControl5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryPanToAsync)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Layers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapLayer>> {
        let this = &::windows::core::Interface::cast::<IMapControl6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Layers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapLayer>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetLayers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<MapLayer>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLayers)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TryGetLocationFromOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetLocationFromOffset)(::core::mem::transmute_copy(this), offset.into_param().abi(), location as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn TryGetLocationFromOffsetWithReferenceSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, offset: Param0, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetLocationFromOffsetWithReferenceSystem)(::core::mem::transmute_copy(this), offset.into_param().abi(), desiredreferencesystem, location as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMapControl7>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Region)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetRegion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControl7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRegion)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanTiltDown(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl8>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanTiltDown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanTiltUp(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl8>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanTiltUp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanZoomIn(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl8>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanZoomIn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanZoomOut(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapControl8>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanZoomOut)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CenterProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CenterProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ChildrenProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChildrenProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ColorSchemeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorSchemeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DesiredPitchProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredPitchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn HeadingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeadingProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LandmarksVisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LandmarksVisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LoadingStatusProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadingStatusProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapServiceTokenProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapServiceTokenProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PedestrianFeaturesVisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PedestrianFeaturesVisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PitchProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PitchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StyleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TrafficFlowVisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrafficFlowVisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransformOriginProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransformOriginProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn WatermarkModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WatermarkModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevelProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevelProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElementsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoutesProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoutesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TileSourcesProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TileSourcesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LocationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn GetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLocation)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IMapControlStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn NormalizedAnchorPointProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NormalizedAnchorPointProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNormalizedAnchorPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        Self::IMapControlStatics(|this| unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNormalizedAnchorPoint)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNormalizedAnchorPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(element: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IMapControlStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetNormalizedAnchorPoint)(::core::mem::transmute_copy(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn BusinessLandmarksVisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarksVisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransitFeaturesVisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturesVisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PanInteractionModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PanInteractionModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RotateInteractionModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotateInteractionModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TiltInteractionModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TiltInteractionModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomInteractionModeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomInteractionModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Is3DSupportedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is3DSupportedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsStreetsideSupportedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStreetsideSupportedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SceneProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SceneProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn BusinessLandmarksEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarksEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransitFeaturesEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturesEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapProjectionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapProjectionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StyleSheetProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSheetProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ViewPaddingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewPaddingProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LayersProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics6(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayersProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RegionProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanTiltDownProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanTiltDownProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanTiltUpProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanTiltUpProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanZoomInProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanZoomInProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CanZoomOutProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapControlStatics8(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanZoomOutProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapControlStatics<R, F: FnOnce(&IMapControlStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics2<R, F: FnOnce(&IMapControlStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics4<R, F: FnOnce(&IMapControlStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics5<R, F: FnOnce(&IMapControlStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics6<R, F: FnOnce(&IMapControlStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics7<R, F: FnOnce(&IMapControlStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlStatics8<R, F: FnOnce(&IMapControlStatics8) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControl, IMapControlStatics8> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControl {}
impl ::core::fmt::Debug for MapControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControl;{42d0b851-5256-4747-9e6c-0d11e966141e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControl {
    type Vtable = IMapControl_Vtbl;
    const IID: ::windows::core::GUID = <IMapControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControl";
}
impl ::core::convert::From<MapControl> for ::windows::core::IUnknown {
    fn from(value: MapControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControl> for ::windows::core::IUnknown {
    fn from(value: &MapControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControl> for ::windows::core::IInspectable {
    fn from(value: MapControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControl> for ::windows::core::IInspectable {
    fn from(value: &MapControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<MapControl> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: MapControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&MapControl> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<MapControl> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: MapControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&MapControl> for super::super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<MapControl> for super::Control {
    fn from(value: MapControl) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapControl> for super::Control {
    fn from(value: &MapControl) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::Control> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::Control> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<MapControl> for super::super::FrameworkElement {
    fn from(value: MapControl) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapControl> for super::super::FrameworkElement {
    fn from(value: &MapControl) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::FrameworkElement> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<MapControl> for super::super::UIElement {
    fn from(value: MapControl) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapControl> for super::super::UIElement {
    fn from(value: &MapControl) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::UIElement> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<MapControl> for super::super::DependencyObject {
    fn from(value: MapControl) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapControl> for super::super::DependencyObject {
    fn from(value: &MapControl) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapControl {}
unsafe impl ::core::marker::Sync for MapControl {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlBusinessLandmarkClickEventArgs(::windows::core::IUnknown);
impl MapControlBusinessLandmarkClickEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlBusinessLandmarkClickEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`, `\"Services_Maps_LocalSearch\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlBusinessLandmarkClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlBusinessLandmarkClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlBusinessLandmarkClickEventArgs {}
impl ::core::fmt::Debug for MapControlBusinessLandmarkClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlBusinessLandmarkClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlBusinessLandmarkClickEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkClickEventArgs;{5e464922-4a1a-4797-beb7-5cf7754cb867})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlBusinessLandmarkClickEventArgs {
    type Vtable = IMapControlBusinessLandmarkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlBusinessLandmarkClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlBusinessLandmarkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkClickEventArgs";
}
impl ::core::convert::From<MapControlBusinessLandmarkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlBusinessLandmarkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlBusinessLandmarkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlBusinessLandmarkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlBusinessLandmarkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlBusinessLandmarkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlBusinessLandmarkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlBusinessLandmarkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlBusinessLandmarkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlBusinessLandmarkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlBusinessLandmarkClickEventArgs {}
unsafe impl ::core::marker::Sync for MapControlBusinessLandmarkClickEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlBusinessLandmarkPointerEnteredEventArgs(::windows::core::IUnknown);
impl MapControlBusinessLandmarkPointerEnteredEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlBusinessLandmarkPointerEnteredEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`, `\"Services_Maps_LocalSearch\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlBusinessLandmarkPointerEnteredEventArgs {}
impl ::core::fmt::Debug for MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlBusinessLandmarkPointerEnteredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlBusinessLandmarkPointerEnteredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkPointerEnteredEventArgs;{5e4081a6-ea98-4f95-8caf-5b42696ff504})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlBusinessLandmarkPointerEnteredEventArgs {
    type Vtable = IMapControlBusinessLandmarkPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlBusinessLandmarkPointerEnteredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlBusinessLandmarkPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkPointerEnteredEventArgs";
}
impl ::core::convert::From<MapControlBusinessLandmarkPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlBusinessLandmarkPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlBusinessLandmarkPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlBusinessLandmarkPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlBusinessLandmarkPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlBusinessLandmarkPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlBusinessLandmarkPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlBusinessLandmarkPointerEnteredEventArgs {}
unsafe impl ::core::marker::Sync for MapControlBusinessLandmarkPointerEnteredEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlBusinessLandmarkPointerExitedEventArgs(::windows::core::IUnknown);
impl MapControlBusinessLandmarkPointerExitedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlBusinessLandmarkPointerExitedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`, `\"Services_Maps_LocalSearch\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlBusinessLandmarkPointerExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlBusinessLandmarkPointerExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlBusinessLandmarkPointerExitedEventArgs {}
impl ::core::fmt::Debug for MapControlBusinessLandmarkPointerExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlBusinessLandmarkPointerExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlBusinessLandmarkPointerExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkPointerExitedEventArgs;{2bb52caf-f24a-46d0-b463-65f719731057})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlBusinessLandmarkPointerExitedEventArgs {
    type Vtable = IMapControlBusinessLandmarkPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlBusinessLandmarkPointerExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlBusinessLandmarkPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkPointerExitedEventArgs";
}
impl ::core::convert::From<MapControlBusinessLandmarkPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlBusinessLandmarkPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlBusinessLandmarkPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlBusinessLandmarkPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlBusinessLandmarkPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlBusinessLandmarkPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlBusinessLandmarkPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlBusinessLandmarkPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlBusinessLandmarkPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlBusinessLandmarkPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlBusinessLandmarkPointerExitedEventArgs {}
unsafe impl ::core::marker::Sync for MapControlBusinessLandmarkPointerExitedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlBusinessLandmarkRightTappedEventArgs(::windows::core::IUnknown);
impl MapControlBusinessLandmarkRightTappedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlBusinessLandmarkRightTappedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`, `\"Services_Maps_LocalSearch\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Services_Maps_LocalSearch"))]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlBusinessLandmarkRightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlBusinessLandmarkRightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlBusinessLandmarkRightTappedEventArgs {}
impl ::core::fmt::Debug for MapControlBusinessLandmarkRightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlBusinessLandmarkRightTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlBusinessLandmarkRightTappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkRightTappedEventArgs;{59ab8ae7-f184-4ab1-b0b0-35c8bf0654b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlBusinessLandmarkRightTappedEventArgs {
    type Vtable = IMapControlBusinessLandmarkRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlBusinessLandmarkRightTappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlBusinessLandmarkRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlBusinessLandmarkRightTappedEventArgs";
}
impl ::core::convert::From<MapControlBusinessLandmarkRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlBusinessLandmarkRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlBusinessLandmarkRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlBusinessLandmarkRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlBusinessLandmarkRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlBusinessLandmarkRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlBusinessLandmarkRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlBusinessLandmarkRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlBusinessLandmarkRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlBusinessLandmarkRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlBusinessLandmarkRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlBusinessLandmarkRightTappedEventArgs {}
unsafe impl ::core::marker::Sync for MapControlBusinessLandmarkRightTappedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlDataHelper(::windows::core::IUnknown);
impl MapControlDataHelper {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BusinessLandmarkClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarkClick)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBusinessLandmarkClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBusinessLandmarkClick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransitFeatureClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeatureClick)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransitFeatureClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransitFeatureClick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BusinessLandmarkRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarkRightTapped)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBusinessLandmarkRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBusinessLandmarkRightTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransitFeatureRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeatureRightTapped)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransitFeatureRightTapped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransitFeatureRightTapped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BusinessLandmarkPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarkPointerEntered)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBusinessLandmarkPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBusinessLandmarkPointerEntered)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransitFeaturePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturePointerEntered)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransitFeaturePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransitFeaturePointerEntered)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BusinessLandmarkPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BusinessLandmarkPointerExited)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBusinessLandmarkPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBusinessLandmarkPointerExited)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransitFeaturePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitFeaturePointerExited)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransitFeaturePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapControlDataHelper2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTransitFeaturePointerExited)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, MapControl>>(map: Param0) -> ::windows::core::Result<MapControlDataHelper> {
        Self::IMapControlDataHelperFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), map.into_param().abi(), &mut result__).from_abi::<MapControlDataHelper>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateMapControl(rasterrendermode: bool) -> ::windows::core::Result<MapControl> {
        Self::IMapControlDataHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMapControl)(::core::mem::transmute_copy(this), rasterrendermode, &mut result__).from_abi::<MapControl>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapControlDataHelperFactory<R, F: FnOnce(&IMapControlDataHelperFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlDataHelper, IMapControlDataHelperFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapControlDataHelperStatics<R, F: FnOnce(&IMapControlDataHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlDataHelper, IMapControlDataHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapControlDataHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlDataHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlDataHelper {}
impl ::core::fmt::Debug for MapControlDataHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlDataHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlDataHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlDataHelper;{8bb0f09c-14ab-486c-9de5-5a5def0205a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlDataHelper {
    type Vtable = IMapControlDataHelper_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlDataHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlDataHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlDataHelper";
}
impl ::core::convert::From<MapControlDataHelper> for ::windows::core::IUnknown {
    fn from(value: MapControlDataHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlDataHelper> for ::windows::core::IUnknown {
    fn from(value: &MapControlDataHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlDataHelper> for ::windows::core::IInspectable {
    fn from(value: MapControlDataHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlDataHelper> for ::windows::core::IInspectable {
    fn from(value: &MapControlDataHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlDataHelper> for super::super::DependencyObject {
    fn from(value: MapControlDataHelper) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapControlDataHelper> for super::super::DependencyObject {
    fn from(value: &MapControlDataHelper) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapControlDataHelper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapControlDataHelper {}
unsafe impl ::core::marker::Sync for MapControlDataHelper {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlTransitFeatureClickEventArgs(::windows::core::IUnknown);
impl MapControlTransitFeatureClickEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlTransitFeatureClickEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlTransitFeatureClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlTransitFeatureClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlTransitFeatureClickEventArgs {}
impl ::core::fmt::Debug for MapControlTransitFeatureClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlTransitFeatureClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlTransitFeatureClickEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlTransitFeatureClickEventArgs;{76179969-b765-4622-b08a-3072745a4541})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlTransitFeatureClickEventArgs {
    type Vtable = IMapControlTransitFeatureClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlTransitFeatureClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlTransitFeatureClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlTransitFeatureClickEventArgs";
}
impl ::core::convert::From<MapControlTransitFeatureClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlTransitFeatureClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeatureClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlTransitFeatureClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlTransitFeatureClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlTransitFeatureClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlTransitFeatureClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlTransitFeatureClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeatureClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlTransitFeatureClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlTransitFeatureClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlTransitFeatureClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlTransitFeatureClickEventArgs {}
unsafe impl ::core::marker::Sync for MapControlTransitFeatureClickEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlTransitFeaturePointerEnteredEventArgs(::windows::core::IUnknown);
impl MapControlTransitFeaturePointerEnteredEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlTransitFeaturePointerEnteredEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlTransitFeaturePointerEnteredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlTransitFeaturePointerEnteredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlTransitFeaturePointerEnteredEventArgs {}
impl ::core::fmt::Debug for MapControlTransitFeaturePointerEnteredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlTransitFeaturePointerEnteredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlTransitFeaturePointerEnteredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlTransitFeaturePointerEnteredEventArgs;{73911a4e-ec4f-479e-94a1-36e081d0d897})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlTransitFeaturePointerEnteredEventArgs {
    type Vtable = IMapControlTransitFeaturePointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlTransitFeaturePointerEnteredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlTransitFeaturePointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlTransitFeaturePointerEnteredEventArgs";
}
impl ::core::convert::From<MapControlTransitFeaturePointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlTransitFeaturePointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeaturePointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlTransitFeaturePointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlTransitFeaturePointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlTransitFeaturePointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlTransitFeaturePointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlTransitFeaturePointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeaturePointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlTransitFeaturePointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlTransitFeaturePointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlTransitFeaturePointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlTransitFeaturePointerEnteredEventArgs {}
unsafe impl ::core::marker::Sync for MapControlTransitFeaturePointerEnteredEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlTransitFeaturePointerExitedEventArgs(::windows::core::IUnknown);
impl MapControlTransitFeaturePointerExitedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlTransitFeaturePointerExitedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlTransitFeaturePointerExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlTransitFeaturePointerExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlTransitFeaturePointerExitedEventArgs {}
impl ::core::fmt::Debug for MapControlTransitFeaturePointerExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlTransitFeaturePointerExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlTransitFeaturePointerExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlTransitFeaturePointerExitedEventArgs;{6a11258d-448d-44e7-bc69-d13d497154e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlTransitFeaturePointerExitedEventArgs {
    type Vtable = IMapControlTransitFeaturePointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlTransitFeaturePointerExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlTransitFeaturePointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlTransitFeaturePointerExitedEventArgs";
}
impl ::core::convert::From<MapControlTransitFeaturePointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlTransitFeaturePointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeaturePointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlTransitFeaturePointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlTransitFeaturePointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlTransitFeaturePointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlTransitFeaturePointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlTransitFeaturePointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeaturePointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlTransitFeaturePointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlTransitFeaturePointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlTransitFeaturePointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlTransitFeaturePointerExitedEventArgs {}
unsafe impl ::core::marker::Sync for MapControlTransitFeaturePointerExitedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapControlTransitFeatureRightTappedEventArgs(::windows::core::IUnknown);
impl MapControlTransitFeatureRightTappedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapControlTransitFeatureRightTappedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapControlTransitFeatureRightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapControlTransitFeatureRightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapControlTransitFeatureRightTappedEventArgs {}
impl ::core::fmt::Debug for MapControlTransitFeatureRightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapControlTransitFeatureRightTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapControlTransitFeatureRightTappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapControlTransitFeatureRightTappedEventArgs;{aea1cc49-a729-4eae-a59a-3ec9a125a028})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapControlTransitFeatureRightTappedEventArgs {
    type Vtable = IMapControlTransitFeatureRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapControlTransitFeatureRightTappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapControlTransitFeatureRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapControlTransitFeatureRightTappedEventArgs";
}
impl ::core::convert::From<MapControlTransitFeatureRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapControlTransitFeatureRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeatureRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapControlTransitFeatureRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapControlTransitFeatureRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapControlTransitFeatureRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapControlTransitFeatureRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapControlTransitFeatureRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapControlTransitFeatureRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapControlTransitFeatureRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapControlTransitFeatureRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapControlTransitFeatureRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapControlTransitFeatureRightTappedEventArgs {}
unsafe impl ::core::marker::Sync for MapControlTransitFeatureRightTappedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapCustomExperience(::windows::core::IUnknown);
impl MapCustomExperience {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapCustomExperience> {
        Self::IMapCustomExperienceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapCustomExperience>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapCustomExperience> {
        Self::IMapCustomExperienceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapCustomExperience>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapCustomExperienceFactory<R, F: FnOnce(&IMapCustomExperienceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapCustomExperience, IMapCustomExperienceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapCustomExperience {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapCustomExperience {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapCustomExperience {}
impl ::core::fmt::Debug for MapCustomExperience {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapCustomExperience").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapCustomExperience {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapCustomExperience;{64592866-14a3-4e5f-8883-8e9c500eeede})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapCustomExperience {
    type Vtable = IMapCustomExperience_Vtbl;
    const IID: ::windows::core::GUID = <IMapCustomExperience as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapCustomExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapCustomExperience";
}
impl ::core::convert::From<MapCustomExperience> for ::windows::core::IUnknown {
    fn from(value: MapCustomExperience) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCustomExperience> for ::windows::core::IUnknown {
    fn from(value: &MapCustomExperience) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapCustomExperience> for ::windows::core::IInspectable {
    fn from(value: MapCustomExperience) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCustomExperience> for ::windows::core::IInspectable {
    fn from(value: &MapCustomExperience) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapCustomExperience> for super::super::DependencyObject {
    fn from(value: MapCustomExperience) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapCustomExperience> for super::super::DependencyObject {
    fn from(value: &MapCustomExperience) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapCustomExperience {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapCustomExperience {}
unsafe impl ::core::marker::Sync for MapCustomExperience {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapCustomExperienceChangedEventArgs(::windows::core::IUnknown);
impl MapCustomExperienceChangedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapCustomExperienceChangedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapCustomExperienceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapCustomExperienceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapCustomExperienceChangedEventArgs {}
impl ::core::fmt::Debug for MapCustomExperienceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapCustomExperienceChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapCustomExperienceChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapCustomExperienceChangedEventArgs;{b9e6fb9b-8fc1-4042-ac34-a61b38bb7514})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapCustomExperienceChangedEventArgs {
    type Vtable = IMapCustomExperienceChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapCustomExperienceChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapCustomExperienceChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapCustomExperienceChangedEventArgs";
}
impl ::core::convert::From<MapCustomExperienceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapCustomExperienceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCustomExperienceChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapCustomExperienceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapCustomExperienceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapCustomExperienceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapCustomExperienceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapCustomExperienceChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapCustomExperienceChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapCustomExperienceChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapCustomExperienceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapCustomExperienceChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapCustomExperienceChangedEventArgs {}
unsafe impl ::core::marker::Sync for MapCustomExperienceChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElement(::windows::core::IUnknown);
impl MapElement {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Visible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapTabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IMapElement2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapTabIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapElement2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMapTabIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapStyleSheetEntry(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapStyleSheetEntry)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapStyleSheetEntry<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMapStyleSheetEntry)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapStyleSheetEntryState(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapStyleSheetEntryState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapStyleSheetEntryState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMapStyleSheetEntryState)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTag)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapElement4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapElement> {
        Self::IMapElementFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapElement>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapElement> {
        Self::IMapElementFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapElement>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn VisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapTabIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapTabIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapStyleSheetEntryProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapStyleSheetEntryProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapStyleSheetEntryStateProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapStyleSheetEntryStateProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TagProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TagProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapElementFactory<R, F: FnOnce(&IMapElementFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement, IMapElementFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapElementStatics<R, F: FnOnce(&IMapElementStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement, IMapElementStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapElementStatics2<R, F: FnOnce(&IMapElementStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement, IMapElementStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapElementStatics3<R, F: FnOnce(&IMapElementStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement, IMapElementStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapElementStatics4<R, F: FnOnce(&IMapElementStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement, IMapElementStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElement {}
impl ::core::fmt::Debug for MapElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElement;{d61fc4df-b245-47f2-9ac2-43c058b1c903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElement {
    type Vtable = IMapElement_Vtbl;
    const IID: ::windows::core::GUID = <IMapElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElement {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElement";
}
impl ::core::convert::From<MapElement> for ::windows::core::IUnknown {
    fn from(value: MapElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElement> for ::windows::core::IUnknown {
    fn from(value: &MapElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElement> for ::windows::core::IInspectable {
    fn from(value: MapElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElement> for ::windows::core::IInspectable {
    fn from(value: &MapElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElement> for super::super::DependencyObject {
    fn from(value: MapElement) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapElement> for super::super::DependencyObject {
    fn from(value: &MapElement) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapElement {}
unsafe impl ::core::marker::Sync for MapElement {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElement3D(::windows::core::IUnknown);
impl MapElement3D {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement3D, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Model(&self) -> ::windows::core::Result<MapModel3D> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Model)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapModel3D>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetModel<'a, Param0: ::windows::core::IntoParam<'a, MapModel3D>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Heading(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Heading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetHeading(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeading)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Pitch(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pitch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetPitch(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPitch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Roll(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Roll)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetRoll(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoll)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Scale(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetScale<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScale)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LocationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElement3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn HeadingProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElement3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeadingProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PitchProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElement3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PitchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RollProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElement3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RollProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ScaleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElement3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapElement3DStatics<R, F: FnOnce(&IMapElement3DStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElement3D, IMapElement3DStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapElement3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElement3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElement3D {}
impl ::core::fmt::Debug for MapElement3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElement3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElement3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElement3D;{827af8d5-3843-48e2-bd00-0f0644fbe6a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElement3D {
    type Vtable = IMapElement3D_Vtbl;
    const IID: ::windows::core::GUID = <IMapElement3D as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElement3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElement3D";
}
impl ::core::convert::From<MapElement3D> for ::windows::core::IUnknown {
    fn from(value: MapElement3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElement3D> for ::windows::core::IUnknown {
    fn from(value: &MapElement3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElement3D> for ::windows::core::IInspectable {
    fn from(value: MapElement3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElement3D> for ::windows::core::IInspectable {
    fn from(value: &MapElement3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElement3D> for MapElement {
    fn from(value: MapElement3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapElement3D> for MapElement {
    fn from(value: &MapElement3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for &MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapElement>::into(self))
    }
}
impl ::core::convert::From<MapElement3D> for super::super::DependencyObject {
    fn from(value: MapElement3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapElement3D> for super::super::DependencyObject {
    fn from(value: &MapElement3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapElement3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapElement3D {}
unsafe impl ::core::marker::Sync for MapElement3D {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementClickEventArgs(::windows::core::IUnknown);
impl MapElementClickEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementClickEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapElement>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementClickEventArgs {}
impl ::core::fmt::Debug for MapElementClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementClickEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementClickEventArgs;{40168a11-d080-4519-99a1-3149fb8999d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementClickEventArgs {
    type Vtable = IMapElementClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementClickEventArgs";
}
impl ::core::convert::From<MapElementClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementClickEventArgs {}
unsafe impl ::core::marker::Sync for MapElementClickEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapElementCollisionBehavior(pub i32);
impl MapElementCollisionBehavior {
    pub const Hide: Self = Self(0i32);
    pub const RemainVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for MapElementCollisionBehavior {}
impl ::core::clone::Clone for MapElementCollisionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapElementCollisionBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapElementCollisionBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapElementCollisionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementCollisionBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementCollisionBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapElementCollisionBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementPointerEnteredEventArgs(::windows::core::IUnknown);
impl MapElementPointerEnteredEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementPointerEnteredEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElement(&self) -> ::windows::core::Result<MapElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElement>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementPointerEnteredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementPointerEnteredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementPointerEnteredEventArgs {}
impl ::core::fmt::Debug for MapElementPointerEnteredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementPointerEnteredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementPointerEnteredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementPointerEnteredEventArgs;{ab85dd4e-91d7-4b31-8f0a-d390c7d3a2ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementPointerEnteredEventArgs {
    type Vtable = IMapElementPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementPointerEnteredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementPointerEnteredEventArgs";
}
impl ::core::convert::From<MapElementPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementPointerEnteredEventArgs {}
unsafe impl ::core::marker::Sync for MapElementPointerEnteredEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementPointerExitedEventArgs(::windows::core::IUnknown);
impl MapElementPointerExitedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementPointerExitedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElement(&self) -> ::windows::core::Result<MapElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElement>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementPointerExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementPointerExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementPointerExitedEventArgs {}
impl ::core::fmt::Debug for MapElementPointerExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementPointerExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementPointerExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementPointerExitedEventArgs;{c1a45af9-60c9-4679-9119-20abc75d931f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementPointerExitedEventArgs {
    type Vtable = IMapElementPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementPointerExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementPointerExitedEventArgs";
}
impl ::core::convert::From<MapElementPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementPointerExitedEventArgs {}
unsafe impl ::core::marker::Sync for MapElementPointerExitedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementsLayer(::windows::core::IUnknown);
impl MapElementsLayer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetMapElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVector<MapElement>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMapElements)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementClick)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementClick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementPointerEntered)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementPointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementPointerEntered)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapElementPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementPointerExited)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapElementPointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapElementPointerExited)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MapContextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapContextRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMapContextRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapContextRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElementsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapElementsLayerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElementsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapElementsLayerStatics<R, F: FnOnce(&IMapElementsLayerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayer, IMapElementsLayerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapElementsLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementsLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementsLayer {}
impl ::core::fmt::Debug for MapElementsLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementsLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementsLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementsLayer;{de79689a-01ef-46f4-ac60-7c200b552610})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementsLayer {
    type Vtable = IMapElementsLayer_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementsLayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementsLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementsLayer";
}
impl ::core::convert::From<MapElementsLayer> for ::windows::core::IUnknown {
    fn from(value: MapElementsLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayer> for ::windows::core::IUnknown {
    fn from(value: &MapElementsLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayer> for ::windows::core::IInspectable {
    fn from(value: MapElementsLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayer> for ::windows::core::IInspectable {
    fn from(value: &MapElementsLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayer> for MapLayer {
    fn from(value: MapElementsLayer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapElementsLayer> for MapLayer {
    fn from(value: &MapElementsLayer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapLayer> for MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, MapLayer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapLayer> for &MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, MapLayer> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapLayer>::into(self))
    }
}
impl ::core::convert::From<MapElementsLayer> for super::super::DependencyObject {
    fn from(value: MapElementsLayer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapElementsLayer> for super::super::DependencyObject {
    fn from(value: &MapElementsLayer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapElementsLayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapElementsLayer {}
unsafe impl ::core::marker::Sync for MapElementsLayer {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementsLayerClickEventArgs(::windows::core::IUnknown);
impl MapElementsLayerClickEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayerClickEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<MapElement>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementsLayerClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementsLayerClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementsLayerClickEventArgs {}
impl ::core::fmt::Debug for MapElementsLayerClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementsLayerClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementsLayerClickEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementsLayerClickEventArgs;{2ca7cf66-af1b-4c05-8c85-f74ae3d4677f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementsLayerClickEventArgs {
    type Vtable = IMapElementsLayerClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementsLayerClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementsLayerClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementsLayerClickEventArgs";
}
impl ::core::convert::From<MapElementsLayerClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementsLayerClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementsLayerClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementsLayerClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementsLayerClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayerClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementsLayerClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementsLayerClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementsLayerClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementsLayerClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementsLayerClickEventArgs {}
unsafe impl ::core::marker::Sync for MapElementsLayerClickEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementsLayerContextRequestedEventArgs(::windows::core::IUnknown);
impl MapElementsLayerContextRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayerContextRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementsLayerContextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementsLayerContextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementsLayerContextRequestedEventArgs {}
impl ::core::fmt::Debug for MapElementsLayerContextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementsLayerContextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementsLayerContextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementsLayerContextRequestedEventArgs;{da45d0b3-7a0e-4758-808b-3a637627eb0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementsLayerContextRequestedEventArgs {
    type Vtable = IMapElementsLayerContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementsLayerContextRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementsLayerContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementsLayerContextRequestedEventArgs";
}
impl ::core::convert::From<MapElementsLayerContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementsLayerContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementsLayerContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementsLayerContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementsLayerContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayerContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementsLayerContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementsLayerContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementsLayerContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementsLayerContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementsLayerContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MapElementsLayerContextRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementsLayerPointerEnteredEventArgs(::windows::core::IUnknown);
impl MapElementsLayerPointerEnteredEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayerPointerEnteredEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElement(&self) -> ::windows::core::Result<MapElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElement>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementsLayerPointerEnteredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementsLayerPointerEnteredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementsLayerPointerEnteredEventArgs {}
impl ::core::fmt::Debug for MapElementsLayerPointerEnteredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementsLayerPointerEnteredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementsLayerPointerEnteredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementsLayerPointerEnteredEventArgs;{757fc032-4694-4404-8c89-348b6b76c5e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementsLayerPointerEnteredEventArgs {
    type Vtable = IMapElementsLayerPointerEnteredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementsLayerPointerEnteredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementsLayerPointerEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementsLayerPointerEnteredEventArgs";
}
impl ::core::convert::From<MapElementsLayerPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementsLayerPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerPointerEnteredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementsLayerPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementsLayerPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementsLayerPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayerPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementsLayerPointerEnteredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerPointerEnteredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementsLayerPointerEnteredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementsLayerPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementsLayerPointerEnteredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementsLayerPointerEnteredEventArgs {}
unsafe impl ::core::marker::Sync for MapElementsLayerPointerEnteredEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapElementsLayerPointerExitedEventArgs(::windows::core::IUnknown);
impl MapElementsLayerPointerExitedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapElementsLayerPointerExitedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapElement(&self) -> ::windows::core::Result<MapElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElement>(result__)
        }
    }
}
impl ::core::clone::Clone for MapElementsLayerPointerExitedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapElementsLayerPointerExitedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapElementsLayerPointerExitedEventArgs {}
impl ::core::fmt::Debug for MapElementsLayerPointerExitedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapElementsLayerPointerExitedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapElementsLayerPointerExitedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapElementsLayerPointerExitedEventArgs;{92f3c6ad-03ed-4c39-af20-2a07ee1ccea6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapElementsLayerPointerExitedEventArgs {
    type Vtable = IMapElementsLayerPointerExitedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapElementsLayerPointerExitedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapElementsLayerPointerExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapElementsLayerPointerExitedEventArgs";
}
impl ::core::convert::From<MapElementsLayerPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapElementsLayerPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerPointerExitedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapElementsLayerPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapElementsLayerPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapElementsLayerPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapElementsLayerPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapElementsLayerPointerExitedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapElementsLayerPointerExitedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapElementsLayerPointerExitedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapElementsLayerPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapElementsLayerPointerExitedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapElementsLayerPointerExitedEventArgs {}
unsafe impl ::core::marker::Sync for MapElementsLayerPointerExitedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapIcon(::windows::core::IUnknown);
impl MapIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapIcon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NormalizedAnchorPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNormalizedAnchorPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNormalizedAnchorPoint)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Image)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetImage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior> {
        let this = &::windows::core::Interface::cast::<IMapIcon2>(self)?;
        unsafe {
            let mut result__: MapElementCollisionBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollisionBehaviorDesired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapElementCollisionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapIcon2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCollisionBehaviorDesired)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LocationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapIconStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TitleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapIconStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TitleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn NormalizedAnchorPointProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapIconStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NormalizedAnchorPointProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CollisionBehaviorDesiredProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapIconStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollisionBehaviorDesiredProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapIconStatics<R, F: FnOnce(&IMapIconStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapIcon, IMapIconStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapIconStatics2<R, F: FnOnce(&IMapIconStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapIcon, IMapIconStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapIcon {}
impl ::core::fmt::Debug for MapIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapIcon;{d2096872-23d9-4a2b-8be0-69f3a85482ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapIcon {
    type Vtable = IMapIcon_Vtbl;
    const IID: ::windows::core::GUID = <IMapIcon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapIcon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapIcon";
}
impl ::core::convert::From<MapIcon> for ::windows::core::IUnknown {
    fn from(value: MapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapIcon> for ::windows::core::IUnknown {
    fn from(value: &MapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapIcon> for ::windows::core::IInspectable {
    fn from(value: MapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapIcon> for ::windows::core::IInspectable {
    fn from(value: &MapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapIcon> for MapElement {
    fn from(value: MapIcon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapIcon> for MapElement {
    fn from(value: &MapIcon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for &MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapElement>::into(self))
    }
}
impl ::core::convert::From<MapIcon> for super::super::DependencyObject {
    fn from(value: MapIcon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapIcon> for super::super::DependencyObject {
    fn from(value: &MapIcon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapIcon {}
unsafe impl ::core::marker::Sync for MapIcon {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapInputEventArgs(::windows::core::IUnknown);
impl MapInputEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapInputEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
}
impl ::core::clone::Clone for MapInputEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapInputEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapInputEventArgs {}
impl ::core::fmt::Debug for MapInputEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapInputEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapInputEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapInputEventArgs;{9fc503a0-a8a2-4394-92e9-2247764f2f49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapInputEventArgs {
    type Vtable = IMapInputEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapInputEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapInputEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapInputEventArgs";
}
impl ::core::convert::From<MapInputEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapInputEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapInputEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapInputEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapInputEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapInputEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapInputEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapInputEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapInputEventArgs> for super::super::DependencyObject {
    fn from(value: MapInputEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapInputEventArgs> for super::super::DependencyObject {
    fn from(value: &MapInputEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapInputEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapInputEventArgs {}
unsafe impl ::core::marker::Sync for MapInputEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapInteractionMode(pub i32);
impl MapInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const GestureOnly: Self = Self(2i32);
    pub const PointerAndKeyboard: Self = Self(2i32);
    pub const ControlOnly: Self = Self(3i32);
    pub const GestureAndControl: Self = Self(4i32);
    pub const PointerKeyboardAndControl: Self = Self(4i32);
    pub const PointerOnly: Self = Self(5i32);
}
impl ::core::marker::Copy for MapInteractionMode {}
impl ::core::clone::Clone for MapInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapInteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapInteractionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapInteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapInteractionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapInteractionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapInteractionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapItemsControl(::windows::core::IUnknown);
impl MapItemsControl {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapItemsControl, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetItemsSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItemsSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Items)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemTemplate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DataTemplate>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetItemTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::DataTemplate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItemTemplate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ItemsSourceProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapItemsControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsSourceProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ItemsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapItemsControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ItemTemplateProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapItemsControlStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemTemplateProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapItemsControlStatics<R, F: FnOnce(&IMapItemsControlStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapItemsControl, IMapItemsControlStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapItemsControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapItemsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapItemsControl {}
impl ::core::fmt::Debug for MapItemsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapItemsControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapItemsControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapItemsControl;{94c2c4d3-b335-42c5-b660-e6a07ec3bddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapItemsControl {
    type Vtable = IMapItemsControl_Vtbl;
    const IID: ::windows::core::GUID = <IMapItemsControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapItemsControl {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapItemsControl";
}
impl ::core::convert::From<MapItemsControl> for ::windows::core::IUnknown {
    fn from(value: MapItemsControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapItemsControl> for ::windows::core::IUnknown {
    fn from(value: &MapItemsControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapItemsControl> for ::windows::core::IInspectable {
    fn from(value: MapItemsControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapItemsControl> for ::windows::core::IInspectable {
    fn from(value: &MapItemsControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapItemsControl> for super::super::DependencyObject {
    fn from(value: MapItemsControl) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapItemsControl> for super::super::DependencyObject {
    fn from(value: &MapItemsControl) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapItemsControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapItemsControl {}
unsafe impl ::core::marker::Sync for MapItemsControl {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapLayer(::windows::core::IUnknown);
impl MapLayer {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapTabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapTabIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMapTabIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Visible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapLayer> {
        Self::IMapLayerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapLayer>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapLayer> {
        Self::IMapLayerFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapLayer>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MapTabIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapLayerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapTabIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn VisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapLayerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapLayerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapLayerFactory<R, F: FnOnce(&IMapLayerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapLayer, IMapLayerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapLayerStatics<R, F: FnOnce(&IMapLayerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapLayer, IMapLayerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapLayer {}
impl ::core::fmt::Debug for MapLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapLayer;{6d0ff9c1-a14d-4f97-8f57-46715b57683a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapLayer {
    type Vtable = IMapLayer_Vtbl;
    const IID: ::windows::core::GUID = <IMapLayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapLayer {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapLayer";
}
impl ::core::convert::From<MapLayer> for ::windows::core::IUnknown {
    fn from(value: MapLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLayer> for ::windows::core::IUnknown {
    fn from(value: &MapLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapLayer> for ::windows::core::IInspectable {
    fn from(value: MapLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapLayer> for ::windows::core::IInspectable {
    fn from(value: &MapLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapLayer> for super::super::DependencyObject {
    fn from(value: MapLayer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapLayer> for super::super::DependencyObject {
    fn from(value: &MapLayer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapLayer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapLayer {}
unsafe impl ::core::marker::Sync for MapLayer {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapLoadingStatus(pub i32);
impl MapLoadingStatus {
    pub const Loading: Self = Self(0i32);
    pub const Loaded: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
    pub const DownloadedMapsManagerUnavailable: Self = Self(3i32);
}
impl ::core::marker::Copy for MapLoadingStatus {}
impl ::core::clone::Clone for MapLoadingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapLoadingStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapLoadingStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapLoadingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLoadingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapLoadingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapLoadingStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapModel3D(::windows::core::IUnknown);
impl MapModel3D {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapModel3D> {
        Self::IMapModel3DFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapModel3D>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapModel3D> {
        Self::IMapModel3DFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapModel3D>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFrom3MFAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStreamReference>>(source: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>> {
        Self::IMapModel3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrom3MFAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFrom3MFWithShadingOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStreamReference>>(source: Param0, shadingoption: MapModel3DShadingOption) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>> {
        Self::IMapModel3DStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrom3MFWithShadingOptionAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), shadingoption, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapModel3DFactory<R, F: FnOnce(&IMapModel3DFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapModel3D, IMapModel3DFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapModel3DStatics<R, F: FnOnce(&IMapModel3DStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapModel3D, IMapModel3DStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapModel3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapModel3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapModel3D {}
impl ::core::fmt::Debug for MapModel3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapModel3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapModel3D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapModel3D;{f8c541a1-ca27-4968-a2bf-9c20f06a0468})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapModel3D {
    type Vtable = IMapModel3D_Vtbl;
    const IID: ::windows::core::GUID = <IMapModel3D as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapModel3D {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapModel3D";
}
impl ::core::convert::From<MapModel3D> for ::windows::core::IUnknown {
    fn from(value: MapModel3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapModel3D> for ::windows::core::IUnknown {
    fn from(value: &MapModel3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapModel3D> for ::windows::core::IInspectable {
    fn from(value: MapModel3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapModel3D> for ::windows::core::IInspectable {
    fn from(value: &MapModel3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapModel3D> for super::super::DependencyObject {
    fn from(value: MapModel3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapModel3D> for super::super::DependencyObject {
    fn from(value: &MapModel3D) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapModel3D {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapModel3D {}
unsafe impl ::core::marker::Sync for MapModel3D {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapModel3DShadingOption(pub i32);
impl MapModel3DShadingOption {
    pub const Default: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for MapModel3DShadingOption {}
impl ::core::clone::Clone for MapModel3DShadingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapModel3DShadingOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapModel3DShadingOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapModel3DShadingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapModel3DShadingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapModel3DShadingOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapModel3DShadingOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapPanInteractionMode(pub i32);
impl MapPanInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for MapPanInteractionMode {}
impl ::core::clone::Clone for MapPanInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapPanInteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapPanInteractionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapPanInteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapPanInteractionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapPanInteractionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapPanInteractionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapPolygon(::windows::core::IUnknown);
impl MapPolygon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapPolygon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopath>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeThickness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeThickness)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeThickness)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeDashed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDashed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetFillColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFillColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn Paths(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>> {
        let this = &::windows::core::Interface::cast::<IMapPolygon2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Paths)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PathProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PathProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeThicknessProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeThicknessProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeDashedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapPolygonStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapPolygonStatics<R, F: FnOnce(&IMapPolygonStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapPolygon, IMapPolygonStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapPolygon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapPolygon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapPolygon {}
impl ::core::fmt::Debug for MapPolygon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapPolygon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapPolygon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapPolygon;{abda2285-4926-4c3a-a5f9-19df7f69db3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapPolygon {
    type Vtable = IMapPolygon_Vtbl;
    const IID: ::windows::core::GUID = <IMapPolygon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapPolygon {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapPolygon";
}
impl ::core::convert::From<MapPolygon> for ::windows::core::IUnknown {
    fn from(value: MapPolygon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapPolygon> for ::windows::core::IUnknown {
    fn from(value: &MapPolygon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapPolygon> for ::windows::core::IInspectable {
    fn from(value: MapPolygon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapPolygon> for ::windows::core::IInspectable {
    fn from(value: &MapPolygon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapPolygon> for MapElement {
    fn from(value: MapPolygon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapPolygon> for MapElement {
    fn from(value: &MapPolygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for &MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapElement>::into(self))
    }
}
impl ::core::convert::From<MapPolygon> for super::super::DependencyObject {
    fn from(value: MapPolygon) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapPolygon> for super::super::DependencyObject {
    fn from(value: &MapPolygon) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapPolygon {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapPolygon {}
unsafe impl ::core::marker::Sync for MapPolygon {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapPolyline(::windows::core::IUnknown);
impl MapPolyline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapPolyline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopath>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeThickness(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeThickness)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeThickness)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeDashed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDashed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PathProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PathProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StrokeDashedProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapPolylineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StrokeDashedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapPolylineStatics<R, F: FnOnce(&IMapPolylineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapPolyline, IMapPolylineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapPolyline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapPolyline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapPolyline {}
impl ::core::fmt::Debug for MapPolyline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapPolyline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapPolyline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapPolyline;{fbad24a2-24df-4a86-8ffa-0f8f4d9ec17d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapPolyline {
    type Vtable = IMapPolyline_Vtbl;
    const IID: ::windows::core::GUID = <IMapPolyline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapPolyline {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapPolyline";
}
impl ::core::convert::From<MapPolyline> for ::windows::core::IUnknown {
    fn from(value: MapPolyline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapPolyline> for ::windows::core::IUnknown {
    fn from(value: &MapPolyline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapPolyline> for ::windows::core::IInspectable {
    fn from(value: MapPolyline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapPolyline> for ::windows::core::IInspectable {
    fn from(value: &MapPolyline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapPolyline> for MapElement {
    fn from(value: MapPolyline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapPolyline> for MapElement {
    fn from(value: &MapPolyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapElement> for &MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, MapElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapElement>::into(self))
    }
}
impl ::core::convert::From<MapPolyline> for super::super::DependencyObject {
    fn from(value: MapPolyline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapPolyline> for super::super::DependencyObject {
    fn from(value: &MapPolyline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapPolyline {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapPolyline {}
unsafe impl ::core::marker::Sync for MapPolyline {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapProjection(pub i32);
impl MapProjection {
    pub const WebMercator: Self = Self(0i32);
    pub const Globe: Self = Self(1i32);
}
impl ::core::marker::Copy for MapProjection {}
impl ::core::clone::Clone for MapProjection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapProjection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapProjection {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapProjection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapRightTappedEventArgs(::windows::core::IUnknown);
impl MapRightTappedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapRightTappedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
}
impl ::core::clone::Clone for MapRightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRightTappedEventArgs {}
impl ::core::fmt::Debug for MapRightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRightTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRightTappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapRightTappedEventArgs;{20943171-6fe8-40a6-ad0e-297379b575a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRightTappedEventArgs {
    type Vtable = IMapRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapRightTappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapRightTappedEventArgs";
}
impl ::core::convert::From<MapRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapRightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapRightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapRightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapRightTappedEventArgs {}
unsafe impl ::core::marker::Sync for MapRightTappedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapRouteView(::windows::core::IUnknown);
impl MapRouteView {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RouteColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RouteColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetRouteColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRouteColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn OutlineColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutlineColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetOutlineColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutlineColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Services_Maps\"`*"]
    #[cfg(feature = "Services_Maps")]
    pub fn Route(&self) -> ::windows::core::Result<super::super::super::super::Services::Maps::MapRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Route)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Services::Maps::MapRoute>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Services_Maps\"`*"]
    #[cfg(feature = "Services_Maps")]
    pub fn CreateInstanceWithMapRoute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Services::Maps::MapRoute>>(route: Param0) -> ::windows::core::Result<MapRouteView> {
        Self::IMapRouteViewFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithMapRoute)(::core::mem::transmute_copy(this), route.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapRouteView>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Services_Maps\"`*"]
    #[cfg(feature = "Services_Maps")]
    pub fn CreateInstanceWithMapRoute_compose<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Services::Maps::MapRoute>, T: ::windows::core::Compose>(route: Param0, compose: T) -> ::windows::core::Result<MapRouteView> {
        Self::IMapRouteViewFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithMapRoute)(::core::mem::transmute_copy(this), route.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapRouteView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapRouteViewFactory<R, F: FnOnce(&IMapRouteViewFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapRouteView, IMapRouteViewFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapRouteView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapRouteView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteView {}
impl ::core::fmt::Debug for MapRouteView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapRouteView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapRouteView;{740eaec5-bacc-41e1-a67e-dd6513832049})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapRouteView {
    type Vtable = IMapRouteView_Vtbl;
    const IID: ::windows::core::GUID = <IMapRouteView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapRouteView {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapRouteView";
}
impl ::core::convert::From<MapRouteView> for ::windows::core::IUnknown {
    fn from(value: MapRouteView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteView> for ::windows::core::IUnknown {
    fn from(value: &MapRouteView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapRouteView> for ::windows::core::IInspectable {
    fn from(value: MapRouteView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapRouteView> for ::windows::core::IInspectable {
    fn from(value: &MapRouteView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapRouteView> for super::super::DependencyObject {
    fn from(value: MapRouteView) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapRouteView> for super::super::DependencyObject {
    fn from(value: &MapRouteView) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapRouteView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapRouteView {}
unsafe impl ::core::marker::Sync for MapRouteView {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapScene(::windows::core::IUnknown);
impl MapScene {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TargetCamera(&self) -> ::windows::core::Result<MapCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCamera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TargetCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCameraChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTargetCameraChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTargetCameraChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromBoundingBox<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>>(bounds: Param0) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBoundingBox)(::core::mem::transmute_copy(this), bounds.into_param().abi(), &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromBoundingBoxWithHeadingAndPitch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>>(bounds: Param0, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBoundingBoxWithHeadingAndPitch)(::core::mem::transmute_copy(this), bounds.into_param().abi(), headingindegrees, pitchindegrees, &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateFromCamera<'a, Param0: ::windows::core::IntoParam<'a, MapCamera>>(camera: Param0) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromCamera)(::core::mem::transmute_copy(this), camera.into_param().abi(), &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocation)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromLocationWithHeadingAndPitch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocationWithHeadingAndPitch)(::core::mem::transmute_copy(this), location.into_param().abi(), headingindegrees, pitchindegrees, &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromLocationAndRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, radiusinmeters: f64) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocationAndRadius)(::core::mem::transmute_copy(this), location.into_param().abi(), radiusinmeters, &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromLocationAndRadiusWithHeadingAndPitch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocationAndRadiusWithHeadingAndPitch)(::core::mem::transmute_copy(this), location.into_param().abi(), radiusinmeters, headingindegrees, pitchindegrees, &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn CreateFromLocations<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>>(locations: Param0) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocations)(::core::mem::transmute_copy(this), locations.into_param().abi(), &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn CreateFromLocationsWithHeadingAndPitch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>>(locations: Param0, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene> {
        Self::IMapSceneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocationsWithHeadingAndPitch)(::core::mem::transmute_copy(this), locations.into_param().abi(), headingindegrees, pitchindegrees, &mut result__).from_abi::<MapScene>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapSceneStatics<R, F: FnOnce(&IMapSceneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapScene, IMapSceneStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapScene {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapScene {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapScene {}
impl ::core::fmt::Debug for MapScene {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapScene").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapScene {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapScene;{8bba10a9-50e7-482c-9789-c688b178ac24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapScene {
    type Vtable = IMapScene_Vtbl;
    const IID: ::windows::core::GUID = <IMapScene as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapScene {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapScene";
}
impl ::core::convert::From<MapScene> for ::windows::core::IUnknown {
    fn from(value: MapScene) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapScene> for ::windows::core::IUnknown {
    fn from(value: &MapScene) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapScene> for ::windows::core::IInspectable {
    fn from(value: MapScene) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapScene> for ::windows::core::IInspectable {
    fn from(value: &MapScene) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapScene> for super::super::DependencyObject {
    fn from(value: MapScene) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapScene> for super::super::DependencyObject {
    fn from(value: &MapScene) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapScene {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapScene {}
unsafe impl ::core::marker::Sync for MapScene {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapStyle(pub i32);
impl MapStyle {
    pub const None: Self = Self(0i32);
    pub const Road: Self = Self(1i32);
    pub const Aerial: Self = Self(2i32);
    pub const AerialWithRoads: Self = Self(3i32);
    pub const Terrain: Self = Self(4i32);
    pub const Aerial3D: Self = Self(5i32);
    pub const Aerial3DWithRoads: Self = Self(6i32);
    pub const Custom: Self = Self(7i32);
}
impl ::core::marker::Copy for MapStyle {}
impl ::core::clone::Clone for MapStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapStyleSheet(::windows::core::IUnknown);
impl MapStyleSheet {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Aerial() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aerial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AerialWithOverlay() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AerialWithOverlay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadLight() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadLight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadDark() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadDark)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadHighContrastLight() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadHighContrastLight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadHighContrastDark() -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadHighContrastDark)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Combine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet>>>(stylesheets: Param0) -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Combine)(::core::mem::transmute_copy(this), stylesheets.into_param().abi(), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ParseFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(styleasjson: Param0) -> ::windows::core::Result<MapStyleSheet> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParseFromJson)(::core::mem::transmute_copy(this), styleasjson.into_param().abi(), &mut result__).from_abi::<MapStyleSheet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TryParseFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(styleasjson: Param0, stylesheet: &mut ::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<bool> {
        Self::IMapStyleSheetStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryParseFromJson)(::core::mem::transmute_copy(this), styleasjson.into_param().abi(), stylesheet as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapStyleSheetStatics<R, F: FnOnce(&IMapStyleSheetStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapStyleSheet, IMapStyleSheetStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapStyleSheet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapStyleSheet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapStyleSheet {}
impl ::core::fmt::Debug for MapStyleSheet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapStyleSheet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapStyleSheet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapStyleSheet;{ae54b2bf-8991-42ed-8d58-20473deede1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapStyleSheet {
    type Vtable = IMapStyleSheet_Vtbl;
    const IID: ::windows::core::GUID = <IMapStyleSheet as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapStyleSheet {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapStyleSheet";
}
impl ::core::convert::From<MapStyleSheet> for ::windows::core::IUnknown {
    fn from(value: MapStyleSheet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapStyleSheet> for ::windows::core::IUnknown {
    fn from(value: &MapStyleSheet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapStyleSheet> for ::windows::core::IInspectable {
    fn from(value: MapStyleSheet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapStyleSheet> for ::windows::core::IInspectable {
    fn from(value: &MapStyleSheet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapStyleSheet> for super::super::DependencyObject {
    fn from(value: MapStyleSheet) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapStyleSheet> for super::super::DependencyObject {
    fn from(value: &MapStyleSheet) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapStyleSheet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapStyleSheet {}
unsafe impl ::core::marker::Sync for MapStyleSheet {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
pub struct MapStyleSheetEntries {}
impl MapStyleSheetEntries {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Area() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Area)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Airport() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Airport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Cemetery() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cemetery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Continent() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Continent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Education() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Education)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IndigenousPeoplesReserve() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndigenousPeoplesReserve)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Island() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Island)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Medical() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Medical)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Military() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Military)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Nautical() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Nautical)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Neighborhood() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Neighborhood)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Runway() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Runway)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Sand() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sand)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ShoppingCenter() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShoppingCenter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Stadium() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stadium)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Vegetation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Vegetation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Forest() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Forest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn GolfCourse() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GolfCourse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Park() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Park)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PlayingField() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlayingField)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Reserve() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reserve)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Point() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Point)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn NaturalPoint() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Peak() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Peak)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn VolcanicPeak() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VolcanicPeak)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn WaterPoint() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WaterPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PointOfInterest() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointOfInterest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Business() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Business)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FoodPoint() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FoodPoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn PopulatedPlace() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PopulatedPlace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Capital() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capital)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AdminDistrictCapital() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdminDistrictCapital)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CountryRegionCapital() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CountryRegionCapital)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadShield() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadShield)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RoadExit() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoadExit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Transit() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Political() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Political)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CountryRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CountryRegion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AdminDistrict() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdminDistrict)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn District() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).District)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Structure() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Structure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Building() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Building)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn EducationBuilding() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EducationBuilding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MedicalBuilding() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MedicalBuilding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TransitBuilding() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitBuilding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Transportation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transportation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Road() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Road)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ControlledAccessHighway() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlledAccessHighway)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn HighSpeedRamp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HighSpeedRamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Highway() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Highway)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn MajorRoad() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MajorRoad)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ArterialRoad() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ArterialRoad)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Street() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Street)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Ramp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ramp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn UnpavedStreet() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnpavedStreet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TollRoad() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TollRoad)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Railway() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Railway)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Trail() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Trail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn WaterRoute() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WaterRoute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Water() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Water)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn River() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).River)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn RouteLine() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RouteLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn WalkingRoute() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WalkingRoute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DrivingRoute() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DrivingRoute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapStyleSheetEntriesStatics<R, F: FnOnce(&IMapStyleSheetEntriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapStyleSheetEntries, IMapStyleSheetEntriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MapStyleSheetEntries {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapStyleSheetEntries";
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
pub struct MapStyleSheetEntryStates {}
impl MapStyleSheetEntryStates {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Disabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntryStatesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Disabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Hover() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntryStatesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Hover)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Selected() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMapStyleSheetEntryStatesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Selected)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapStyleSheetEntryStatesStatics<R, F: FnOnce(&IMapStyleSheetEntryStatesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapStyleSheetEntryStates, IMapStyleSheetEntryStatesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MapStyleSheetEntryStates {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapStyleSheetEntryStates";
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTargetCameraChangedEventArgs(::windows::core::IUnknown);
impl MapTargetCameraChangedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTargetCameraChangedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Camera(&self) -> ::windows::core::Result<MapCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Camera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCamera>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason> {
        let this = &::windows::core::Interface::cast::<IMapTargetCameraChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: MapCameraChangeReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapCameraChangeReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MapTargetCameraChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTargetCameraChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTargetCameraChangedEventArgs {}
impl ::core::fmt::Debug for MapTargetCameraChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTargetCameraChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTargetCameraChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTargetCameraChangedEventArgs;{dbf00472-e953-4fa8-97d0-ea86359057cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTargetCameraChangedEventArgs {
    type Vtable = IMapTargetCameraChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapTargetCameraChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTargetCameraChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTargetCameraChangedEventArgs";
}
impl ::core::convert::From<MapTargetCameraChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapTargetCameraChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTargetCameraChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapTargetCameraChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTargetCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTargetCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTargetCameraChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapTargetCameraChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTargetCameraChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapTargetCameraChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTargetCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTargetCameraChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTargetCameraChangedEventArgs {}
unsafe impl ::core::marker::Sync for MapTargetCameraChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapTileAnimationState(pub i32);
impl MapTileAnimationState {
    pub const Stopped: Self = Self(0i32);
    pub const Paused: Self = Self(1i32);
    pub const Playing: Self = Self(2i32);
}
impl ::core::marker::Copy for MapTileAnimationState {}
impl ::core::clone::Clone for MapTileAnimationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapTileAnimationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapTileAnimationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapTileAnimationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileAnimationState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileAnimationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapTileAnimationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileBitmapRequest(::windows::core::IUnknown);
impl MapTileBitmapRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileBitmapRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPixelData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPixelData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<MapTileBitmapRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileBitmapRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MapTileBitmapRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileBitmapRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileBitmapRequest {}
impl ::core::fmt::Debug for MapTileBitmapRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileBitmapRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileBitmapRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequest;{46733fbc-d89d-472b-b5f6-d7066b0584f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileBitmapRequest {
    type Vtable = IMapTileBitmapRequest_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileBitmapRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileBitmapRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequest";
}
impl ::core::convert::From<MapTileBitmapRequest> for ::windows::core::IUnknown {
    fn from(value: MapTileBitmapRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequest> for ::windows::core::IUnknown {
    fn from(value: &MapTileBitmapRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileBitmapRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileBitmapRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileBitmapRequest> for ::windows::core::IInspectable {
    fn from(value: MapTileBitmapRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequest> for ::windows::core::IInspectable {
    fn from(value: &MapTileBitmapRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileBitmapRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileBitmapRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileBitmapRequest {}
unsafe impl ::core::marker::Sync for MapTileBitmapRequest {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileBitmapRequestDeferral(::windows::core::IUnknown);
impl MapTileBitmapRequestDeferral {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileBitmapRequestDeferral, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MapTileBitmapRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileBitmapRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileBitmapRequestDeferral {}
impl ::core::fmt::Debug for MapTileBitmapRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileBitmapRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileBitmapRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequestDeferral;{fe370542-a4ac-4efa-9665-0490b0cafdd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileBitmapRequestDeferral {
    type Vtable = IMapTileBitmapRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileBitmapRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileBitmapRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequestDeferral";
}
impl ::core::convert::From<MapTileBitmapRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: MapTileBitmapRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &MapTileBitmapRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileBitmapRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileBitmapRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileBitmapRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: MapTileBitmapRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &MapTileBitmapRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileBitmapRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileBitmapRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileBitmapRequestDeferral {}
unsafe impl ::core::marker::Sync for MapTileBitmapRequestDeferral {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileBitmapRequestedEventArgs(::windows::core::IUnknown);
impl MapTileBitmapRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileBitmapRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn X(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Y(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<MapTileBitmapRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileBitmapRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FrameIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IMapTileBitmapRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for MapTileBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileBitmapRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileBitmapRequestedEventArgs {}
impl ::core::fmt::Debug for MapTileBitmapRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileBitmapRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileBitmapRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequestedEventArgs;{337f691d-9b02-4aa2-8b1e-cc4d91719bf3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileBitmapRequestedEventArgs {
    type Vtable = IMapTileBitmapRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileBitmapRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileBitmapRequestedEventArgs";
}
impl ::core::convert::From<MapTileBitmapRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapTileBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapTileBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileBitmapRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapTileBitmapRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileBitmapRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapTileBitmapRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileBitmapRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileBitmapRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MapTileBitmapRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileDataSource(::windows::core::IUnknown);
impl MapTileDataSource {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapTileDataSource> {
        Self::IMapTileDataSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileDataSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapTileDataSource> {
        Self::IMapTileDataSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileDataSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapTileDataSourceFactory<R, F: FnOnce(&IMapTileDataSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileDataSource, IMapTileDataSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapTileDataSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileDataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileDataSource {}
impl ::core::fmt::Debug for MapTileDataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileDataSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileDataSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileDataSource;{c03d9f5e-be1f-4c69-9969-79467a513c38})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileDataSource {
    type Vtable = IMapTileDataSource_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileDataSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileDataSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileDataSource";
}
impl ::core::convert::From<MapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: MapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileDataSource> for ::windows::core::IUnknown {
    fn from(value: &MapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: MapTileDataSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileDataSource> for ::windows::core::IInspectable {
    fn from(value: &MapTileDataSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileDataSource> for super::super::DependencyObject {
    fn from(value: MapTileDataSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapTileDataSource> for super::super::DependencyObject {
    fn from(value: &MapTileDataSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapTileDataSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapTileDataSource {}
unsafe impl ::core::marker::Sync for MapTileDataSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapTileLayer(pub i32);
impl MapTileLayer {
    pub const LabelOverlay: Self = Self(0i32);
    pub const RoadOverlay: Self = Self(1i32);
    pub const AreaOverlay: Self = Self(2i32);
    pub const BackgroundOverlay: Self = Self(3i32);
    pub const BackgroundReplacement: Self = Self(4i32);
}
impl ::core::marker::Copy for MapTileLayer {}
impl ::core::clone::Clone for MapTileLayer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapTileLayer {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapTileLayer {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapTileLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapTileLayer;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileSource(::windows::core::IUnknown);
impl MapTileSource {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DataSource(&self) -> ::windows::core::Result<MapTileDataSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileDataSource>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetDataSource<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Layer(&self) -> ::windows::core::Result<MapTileLayer> {
        let this = self;
        unsafe {
            let mut result__: MapTileLayer = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Layer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileLayer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetLayer(&self, value: MapTileLayer) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLayer)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevelRange(&self) -> ::windows::core::Result<MapZoomLevelRange> {
        let this = self;
        unsafe {
            let mut result__: MapZoomLevelRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevelRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapZoomLevelRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZoomLevelRange<'a, Param0: ::windows::core::IntoParam<'a, MapZoomLevelRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZoomLevelRange)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bounds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::GeoboundingBox>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBounds)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AllowOverstretch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowOverstretch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetAllowOverstretch(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowOverstretch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsFadingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFadingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetIsFadingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsFadingEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsTransparencyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransparencyEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetIsTransparencyEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTransparencyEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsRetryEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRetryEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetIsRetryEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRetryEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TilePixelSize(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TilePixelSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetTilePixelSize(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTilePixelSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Visible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AnimationState(&self) -> ::windows::core::Result<MapTileAnimationState> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe {
            let mut result__: MapTileAnimationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnimationState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileAnimationState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AutoPlay(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoPlay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoPlay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FrameCount(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetFrameCount(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFrameCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFrameDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFrameDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Play(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Play)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMapTileSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn new() -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithDataSource<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>>(datasource: Param0) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSource)(::core::mem::transmute_copy(this), datasource.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithDataSource_compose<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, T: ::windows::core::Compose>(datasource: Param0, compose: T) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSource)(::core::mem::transmute_copy(this), datasource.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithDataSourceAndZoomRange<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>>(datasource: Param0, zoomlevelrange: Param1) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceAndZoomRange)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithDataSourceAndZoomRange_compose<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>, T: ::windows::core::Compose>(datasource: Param0, zoomlevelrange: Param1, compose: T) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceAndZoomRange)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithDataSourceZoomRangeAndBounds<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>>(datasource: Param0, zoomlevelrange: Param1, bounds: Param2) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceZoomRangeAndBounds)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), bounds.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithDataSourceZoomRangeAndBounds_compose<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>, T: ::windows::core::Compose>(datasource: Param0, zoomlevelrange: Param1, bounds: Param2, compose: T) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceZoomRangeAndBounds)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), bounds.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>>(datasource: Param0, zoomlevelrange: Param1, bounds: Param2, tilesizeinpixels: i32) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), bounds.into_param().abi(), tilesizeinpixels, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize_compose<'a, Param0: ::windows::core::IntoParam<'a, MapTileDataSource>, Param1: ::windows::core::IntoParam<'a, MapZoomLevelRange>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::GeoboundingBox>, T: ::windows::core::Compose>(datasource: Param0, zoomlevelrange: Param1, bounds: Param2, tilesizeinpixels: i32, compose: T) -> ::windows::core::Result<MapTileSource> {
        Self::IMapTileSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize)(::core::mem::transmute_copy(this), datasource.into_param().abi(), zoomlevelrange.into_param().abi(), bounds.into_param().abi(), tilesizeinpixels, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<MapTileSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn DataSourceProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataSourceProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn LayerProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LayerProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevelRangeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevelRangeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn BoundsProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BoundsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AllowOverstretchProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowOverstretchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsFadingEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFadingEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsTransparencyEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransparencyEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn IsRetryEnabledProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRetryEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZIndexProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn TilePixelSizeProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TilePixelSizeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn VisibleProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VisibleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AnimationStateProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnimationStateProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AutoPlayProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoPlayProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FrameCountProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameCountProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FrameDurationProperty() -> ::windows::core::Result<super::super::DependencyProperty> {
        Self::IMapTileSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameDurationProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapTileSourceFactory<R, F: FnOnce(&IMapTileSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileSource, IMapTileSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapTileSourceStatics<R, F: FnOnce(&IMapTileSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileSource, IMapTileSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMapTileSourceStatics2<R, F: FnOnce(&IMapTileSourceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileSource, IMapTileSourceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MapTileSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileSource {}
impl ::core::fmt::Debug for MapTileSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileSource;{88a76e4e-2fdf-4567-9255-1100519c8d62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileSource {
    type Vtable = IMapTileSource_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileSource {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileSource";
}
impl ::core::convert::From<MapTileSource> for ::windows::core::IUnknown {
    fn from(value: MapTileSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileSource> for ::windows::core::IUnknown {
    fn from(value: &MapTileSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileSource> for ::windows::core::IInspectable {
    fn from(value: MapTileSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileSource> for ::windows::core::IInspectable {
    fn from(value: &MapTileSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileSource> for super::super::DependencyObject {
    fn from(value: MapTileSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MapTileSource> for super::super::DependencyObject {
    fn from(value: &MapTileSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &MapTileSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MapTileSource {}
unsafe impl ::core::marker::Sync for MapTileSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileUriRequest(::windows::core::IUnknown);
impl MapTileUriRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileUriRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<MapTileUriRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileUriRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MapTileUriRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileUriRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileUriRequest {}
impl ::core::fmt::Debug for MapTileUriRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileUriRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileUriRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileUriRequest;{17402335-3127-45b8-87a7-99f87d4e2745})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileUriRequest {
    type Vtable = IMapTileUriRequest_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileUriRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileUriRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileUriRequest";
}
impl ::core::convert::From<MapTileUriRequest> for ::windows::core::IUnknown {
    fn from(value: MapTileUriRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequest> for ::windows::core::IUnknown {
    fn from(value: &MapTileUriRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileUriRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileUriRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileUriRequest> for ::windows::core::IInspectable {
    fn from(value: MapTileUriRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequest> for ::windows::core::IInspectable {
    fn from(value: &MapTileUriRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileUriRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileUriRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileUriRequest {}
unsafe impl ::core::marker::Sync for MapTileUriRequest {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileUriRequestDeferral(::windows::core::IUnknown);
impl MapTileUriRequestDeferral {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileUriRequestDeferral, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MapTileUriRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileUriRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileUriRequestDeferral {}
impl ::core::fmt::Debug for MapTileUriRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileUriRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileUriRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileUriRequestDeferral;{c117ade0-bf3e-4c51-8faa-4b593cf68eb2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileUriRequestDeferral {
    type Vtable = IMapTileUriRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileUriRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileUriRequestDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileUriRequestDeferral";
}
impl ::core::convert::From<MapTileUriRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: MapTileUriRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &MapTileUriRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileUriRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileUriRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileUriRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: MapTileUriRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &MapTileUriRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileUriRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileUriRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileUriRequestDeferral {}
unsafe impl ::core::marker::Sync for MapTileUriRequestDeferral {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct MapTileUriRequestedEventArgs(::windows::core::IUnknown);
impl MapTileUriRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MapTileUriRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn X(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).X)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Y(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Y)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomLevel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<MapTileUriRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MapTileUriRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn FrameIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IMapTileUriRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for MapTileUriRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapTileUriRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapTileUriRequestedEventArgs {}
impl ::core::fmt::Debug for MapTileUriRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapTileUriRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapTileUriRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.MapTileUriRequestedEventArgs;{d2147b43-1bbf-4b98-8dd3-b7834e407e0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapTileUriRequestedEventArgs {
    type Vtable = IMapTileUriRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMapTileUriRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapTileUriRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.MapTileUriRequestedEventArgs";
}
impl ::core::convert::From<MapTileUriRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MapTileUriRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MapTileUriRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapTileUriRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapTileUriRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapTileUriRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MapTileUriRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapTileUriRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MapTileUriRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapTileUriRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapTileUriRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MapTileUriRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MapTileUriRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapVisibleRegionKind(pub i32);
impl MapVisibleRegionKind {
    pub const Near: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
}
impl ::core::marker::Copy for MapVisibleRegionKind {}
impl ::core::clone::Clone for MapVisibleRegionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapVisibleRegionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapVisibleRegionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapVisibleRegionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapVisibleRegionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapVisibleRegionKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapVisibleRegionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MapWatermarkMode(pub i32);
impl MapWatermarkMode {
    pub const Automatic: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MapWatermarkMode {}
impl ::core::clone::Clone for MapWatermarkMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapWatermarkMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MapWatermarkMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MapWatermarkMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapWatermarkMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapWatermarkMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Maps.MapWatermarkMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
pub struct MapZoomLevelRange {
    pub Min: f64,
    pub Max: f64,
}
impl ::core::marker::Copy for MapZoomLevelRange {}
impl ::core::clone::Clone for MapZoomLevelRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapZoomLevelRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapZoomLevelRange").field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
unsafe impl ::windows::core::Abi for MapZoomLevelRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MapZoomLevelRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Controls.Maps.MapZoomLevelRange;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for MapZoomLevelRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapZoomLevelRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapZoomLevelRange {}
impl ::core::default::Default for MapZoomLevelRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct StreetsideExperience(::windows::core::IUnknown);
impl StreetsideExperience {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn AddressTextVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddressTextVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetAddressTextVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddressTextVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CursorVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetCursorVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCursorVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn OverviewMapVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverviewMapVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetOverviewMapVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOverviewMapVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn StreetLabelsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreetLabelsVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetStreetLabelsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStreetLabelsVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ExitButtonVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitButtonVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetExitButtonVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExitButtonVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn ZoomButtonsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomButtonsVisible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn SetZoomButtonsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetZoomButtonsVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithPanorama<'a, Param0: ::windows::core::IntoParam<'a, StreetsidePanorama>>(panorama: Param0) -> ::windows::core::Result<StreetsideExperience> {
        Self::IStreetsideExperienceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithPanorama)(::core::mem::transmute_copy(this), panorama.into_param().abi(), &mut result__).from_abi::<StreetsideExperience>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
    pub fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView<'a, Param0: ::windows::core::IntoParam<'a, StreetsidePanorama>>(panorama: Param0, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<StreetsideExperience> {
        Self::IStreetsideExperienceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithPanoramaHeadingPitchAndFieldOfView)(::core::mem::transmute_copy(this), panorama.into_param().abi(), headingindegrees, pitchindegrees, fieldofviewindegrees, &mut result__).from_abi::<StreetsideExperience>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStreetsideExperienceFactory<R, F: FnOnce(&IStreetsideExperienceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreetsideExperience, IStreetsideExperienceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StreetsideExperience {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreetsideExperience {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreetsideExperience {}
impl ::core::fmt::Debug for StreetsideExperience {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreetsideExperience").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreetsideExperience {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.StreetsideExperience;{a558aec9-e30c-46c8-8116-484691675558})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StreetsideExperience {
    type Vtable = IStreetsideExperience_Vtbl;
    const IID: ::windows::core::GUID = <IStreetsideExperience as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StreetsideExperience {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.StreetsideExperience";
}
impl ::core::convert::From<StreetsideExperience> for ::windows::core::IUnknown {
    fn from(value: StreetsideExperience) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreetsideExperience> for ::windows::core::IUnknown {
    fn from(value: &StreetsideExperience) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreetsideExperience> for ::windows::core::IInspectable {
    fn from(value: StreetsideExperience) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreetsideExperience> for ::windows::core::IInspectable {
    fn from(value: &StreetsideExperience) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreetsideExperience> for MapCustomExperience {
    fn from(value: StreetsideExperience) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StreetsideExperience> for MapCustomExperience {
    fn from(value: &StreetsideExperience) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapCustomExperience> for StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, MapCustomExperience> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, MapCustomExperience> for &StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, MapCustomExperience> {
        ::windows::core::Param::Owned(::core::convert::Into::<MapCustomExperience>::into(self))
    }
}
impl ::core::convert::From<StreetsideExperience> for super::super::DependencyObject {
    fn from(value: StreetsideExperience) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StreetsideExperience> for super::super::DependencyObject {
    fn from(value: &StreetsideExperience) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &StreetsideExperience {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for StreetsideExperience {}
unsafe impl ::core::marker::Sync for StreetsideExperience {}
#[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`*"]
#[repr(transparent)]
pub struct StreetsidePanorama(::windows::core::IUnknown);
impl StreetsidePanorama {
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Location)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindNearbyWithLocationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>> {
        Self::IStreetsidePanoramaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNearbyWithLocationAsync)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Maps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindNearbyWithLocationAndRadiusAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Devices::Geolocation::Geopoint>>(location: Param0, radiusinmeters: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>> {
        Self::IStreetsidePanoramaStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNearbyWithLocationAndRadiusAsync)(::core::mem::transmute_copy(this), location.into_param().abi(), radiusinmeters, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStreetsidePanoramaStatics<R, F: FnOnce(&IStreetsidePanoramaStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StreetsidePanorama, IStreetsidePanoramaStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StreetsidePanorama {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StreetsidePanorama {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreetsidePanorama {}
impl ::core::fmt::Debug for StreetsidePanorama {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreetsidePanorama").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreetsidePanorama {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Maps.StreetsidePanorama;{6fe00fd8-ad60-4664-b539-b1069f16c5af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StreetsidePanorama {
    type Vtable = IStreetsidePanorama_Vtbl;
    const IID: ::windows::core::GUID = <IStreetsidePanorama as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StreetsidePanorama {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Maps.StreetsidePanorama";
}
impl ::core::convert::From<StreetsidePanorama> for ::windows::core::IUnknown {
    fn from(value: StreetsidePanorama) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreetsidePanorama> for ::windows::core::IUnknown {
    fn from(value: &StreetsidePanorama) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreetsidePanorama> for ::windows::core::IInspectable {
    fn from(value: StreetsidePanorama) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StreetsidePanorama> for ::windows::core::IInspectable {
    fn from(value: &StreetsidePanorama) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StreetsidePanorama> for super::super::DependencyObject {
    fn from(value: StreetsidePanorama) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StreetsidePanorama> for super::super::DependencyObject {
    fn from(value: &StreetsidePanorama) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &StreetsidePanorama {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for StreetsidePanorama {}
unsafe impl ::core::marker::Sync for StreetsidePanorama {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
