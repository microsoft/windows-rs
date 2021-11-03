#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpCacheDirectiveHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpCacheDirectiveHeaderValueCollection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn MaxAge(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetMaxAge<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn MaxStale(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetMaxStale<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn MinFresh(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetMinFresh<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SharedMaxAge(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetSharedMaxAge<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpNameValueHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpNameValueHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpNameValueHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpNameValueHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpNameValueHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection;{9a586b89-d5d0-4fbe-bd9d-b5b3636811b4})");
}
unsafe impl ::windows::runtime::Interface for HttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2589485961, 54736, 20414, [189, 157, 181, 179, 99, 104, 17, 180]);
}
impl ::windows::runtime::RuntimeName for HttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection";
}
impl ::std::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpCacheDirectiveHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpChallengeHeaderValue(::windows::runtime::IInspectable);
impl HttpChallengeHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Scheme(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Token(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromScheme<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0) -> ::windows::runtime::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0, token: Param1) -> ::windows::runtime::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), scheme.into_param().abi(), token.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, challengeheadervalue: &mut ::std::option::Option<HttpChallengeHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), challengeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpChallengeHeaderValueFactory<R, F: FnOnce(&IHttpChallengeHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpChallengeHeaderValueStatics<R, F: FnOnce(&IHttpChallengeHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpChallengeHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValue;{393361af-0f7d-4820-9fdd-a2b956eeaeab})");
}
unsafe impl ::windows::runtime::Interface for HttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(959668655, 3965, 18464, [159, 221, 162, 185, 86, 238, 174, 171]);
}
impl ::windows::runtime::RuntimeName for HttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValue";
}
impl ::std::convert::From<HttpChallengeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpChallengeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpChallengeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpChallengeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpChallengeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpChallengeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpChallengeHeaderValue {}
unsafe impl ::std::marker::Sync for HttpChallengeHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpChallengeHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpChallengeHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpChallengeHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpChallengeHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpChallengeHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpChallengeHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpChallengeHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpChallengeHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpChallengeHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpChallengeHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection;{ca9e5f81-aee0-4353-a10b-e625babd64c2})");
}
unsafe impl ::windows::runtime::Interface for HttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3399376769, 44768, 17235, [161, 11, 230, 37, 186, 189, 100, 194]);
}
impl ::windows::runtime::RuntimeName for HttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection";
}
impl ::std::convert::From<HttpChallengeHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpChallengeHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpChallengeHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpChallengeHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpChallengeHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpChallengeHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpConnectionOptionHeaderValue(::windows::runtime::IInspectable);
impl HttpConnectionOptionHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Token(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(token: Param0) -> ::windows::runtime::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, connectionoptionheadervalue: &mut ::std::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), connectionoptionheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpConnectionOptionHeaderValueFactory<R, F: FnOnce(&IHttpConnectionOptionHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpConnectionOptionHeaderValueStatics<R, F: FnOnce(&IHttpConnectionOptionHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpConnectionOptionHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue;{cb4af27a-4e90-45eb-8dcd-fd1408f4c44f})");
}
unsafe impl ::windows::runtime::Interface for HttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3410686586, 20112, 17899, [141, 205, 253, 20, 8, 244, 196, 79]);
}
impl ::windows::runtime::RuntimeName for HttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue";
}
impl ::std::convert::From<HttpConnectionOptionHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpConnectionOptionHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpConnectionOptionHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpConnectionOptionHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpConnectionOptionHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpConnectionOptionHeaderValue {}
unsafe impl ::std::marker::Sync for HttpConnectionOptionHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpConnectionOptionHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpConnectionOptionHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpConnectionOptionHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpConnectionOptionHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpConnectionOptionHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection;{e4f56c1d-5142-4e00-8e0f-019509337629})");
}
unsafe impl ::windows::runtime::Interface for HttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3841289245, 20802, 19968, [142, 15, 1, 149, 9, 51, 118, 41]);
}
impl ::windows::runtime::RuntimeName for HttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection";
}
impl ::std::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpConnectionOptionHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpConnectionOptionHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentCodingHeaderValue(::windows::runtime::IInspectable);
impl HttpContentCodingHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentCoding(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contentcoding: Param0) -> ::windows::runtime::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentcoding.into_param().abi(), &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, contentcodingheadervalue: &mut ::std::option::Option<HttpContentCodingHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), contentcodingheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpContentCodingHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentCodingHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentCodingHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValue;{bcf7f92a-9376-4d85-bccc-9f4f9acab434})");
}
unsafe impl ::windows::runtime::Interface for HttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3170367786, 37750, 19845, [188, 204, 159, 79, 154, 202, 180, 52]);
}
impl ::windows::runtime::RuntimeName for HttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValue";
}
impl ::std::convert::From<HttpContentCodingHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentCodingHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentCodingHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentCodingHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentCodingHeaderValue {}
unsafe impl ::std::marker::Sync for HttpContentCodingHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentCodingHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpContentCodingHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpContentCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpContentCodingHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpContentCodingHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpContentCodingHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentCodingHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection;{7d221721-a6db-436e-8e83-91596192819c})");
}
unsafe impl ::windows::runtime::Interface for HttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2099386145, 42715, 17262, [142, 131, 145, 89, 97, 146, 129, 156]);
}
impl ::windows::runtime::RuntimeName for HttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection";
}
impl ::std::convert::From<HttpContentCodingHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentCodingHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentCodingHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpContentCodingHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentCodingWithQualityHeaderValue(::windows::runtime::IInspectable);
impl HttpContentCodingWithQualityHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentCoding(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Quality(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contentcoding: Param0) -> ::windows::runtime::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentcoding.into_param().abi(), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromValueWithQuality<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contentcoding: Param0, quality: f64) -> ::windows::runtime::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contentcoding.into_param().abi(), quality, &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, contentcodingwithqualityheadervalue: &mut ::std::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), contentcodingwithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpContentCodingWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentCodingWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue;{94531cd5-8b13-4d73-8651-f76b38f88495})");
}
unsafe impl ::windows::runtime::Interface for HttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488474837, 35603, 19827, [134, 81, 247, 107, 56, 248, 132, 149]);
}
impl ::windows::runtime::RuntimeName for HttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue";
}
impl ::std::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentCodingWithQualityHeaderValue {}
unsafe impl ::std::marker::Sync for HttpContentCodingWithQualityHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentCodingWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpContentCodingWithQualityHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpContentCodingWithQualityHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpContentCodingWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpContentCodingWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection;{7c0d753e-e899-4378-b5c8-412d820711cc})");
}
unsafe impl ::windows::runtime::Interface for HttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2081256766, 59545, 17272, [181, 200, 65, 45, 130, 7, 17, 204]);
}
impl ::windows::runtime::RuntimeName for HttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection";
}
impl ::std::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpContentCodingWithQualityHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentDispositionHeaderValue(::windows::runtime::IInspectable);
impl HttpContentDispositionHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn DispositionType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetDispositionType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn FileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetFileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn FileNameStar(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetFileNameStar<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(dispositiontype: Param0) -> ::windows::runtime::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dispositiontype.into_param().abi(), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, contentdispositionheadervalue: &mut ::std::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), contentdispositionheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpContentDispositionHeaderValueFactory<R, F: FnOnce(&IHttpContentDispositionHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentDispositionHeaderValueStatics<R, F: FnOnce(&IHttpContentDispositionHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentDispositionHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentDispositionHeaderValue;{f2a2eedc-2629-4b49-9908-96a168e9365e})");
}
unsafe impl ::windows::runtime::Interface for HttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4070764252, 9769, 19273, [153, 8, 150, 161, 104, 233, 54, 94]);
}
impl ::windows::runtime::RuntimeName for HttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentDispositionHeaderValue";
}
impl ::std::convert::From<HttpContentDispositionHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentDispositionHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentDispositionHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentDispositionHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentDispositionHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentDispositionHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentDispositionHeaderValue {}
unsafe impl ::std::marker::Sync for HttpContentDispositionHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentHeaderCollection(::windows::runtime::IInspectable);
impl HttpContentHeaderCollection {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentHeaderCollection, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentDisposition(&self) -> ::windows::runtime::Result<HttpContentDispositionHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetContentDisposition<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentDispositionHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentEncoding(&self) -> ::windows::runtime::Result<HttpContentCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentLanguage(&self) -> ::windows::runtime::Result<HttpLanguageHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpLanguageHeaderValueCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ContentLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetContentLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ContentLocation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetContentLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Web_Http_Headers`, `Storage_Streams`*"]
    pub fn ContentMD5(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Web_Http_Headers`, `Storage_Streams`*"]
    pub fn SetContentMD5<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentRange(&self) -> ::windows::runtime::Result<HttpContentRangeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetContentRange<'a, Param0: ::windows::runtime::IntoParam<'a, HttpContentRangeHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<HttpMediaTypeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetContentType<'a, Param0: ::windows::runtime::IntoParam<'a, HttpMediaTypeHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Expires(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetExpires<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn LastModified(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetLastModified<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentHeaderCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentHeaderCollection;{40612a44-47ae-4b7e-9124-69628b64aa18})");
}
unsafe impl ::windows::runtime::Interface for HttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1080109636, 18350, 19326, [145, 36, 105, 98, 139, 100, 170, 24]);
}
impl ::windows::runtime::RuntimeName for HttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentHeaderCollection";
}
impl ::std::convert::From<HttpContentHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentHeaderCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentHeaderCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentHeaderCollection {}
unsafe impl ::std::marker::Sync for HttpContentHeaderCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpContentRangeHeaderValue(::windows::runtime::IInspectable);
impl HttpContentRangeHeaderValue {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn FirstBytePosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn LastBytePosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Unit(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromLength(length: u64) -> ::windows::runtime::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), length, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromRange(from: u64, to: u64) -> ::windows::runtime::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), from, to, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromRangeWithLength(from: u64, to: u64, length: u64) -> ::windows::runtime::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), from, to, length, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, contentrangeheadervalue: &mut ::std::option::Option<HttpContentRangeHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), contentrangeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpContentRangeHeaderValueFactory<R, F: FnOnce(&IHttpContentRangeHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentRangeHeaderValueStatics<R, F: FnOnce(&IHttpContentRangeHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpContentRangeHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentRangeHeaderValue;{04d967d3-a4f6-495c-9530-8579fcba8aa9})");
}
unsafe impl ::windows::runtime::Interface for HttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(81356755, 42230, 18780, [149, 48, 133, 121, 252, 186, 138, 169]);
}
impl ::windows::runtime::RuntimeName for HttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentRangeHeaderValue";
}
impl ::std::convert::From<HttpContentRangeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpContentRangeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpContentRangeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpContentRangeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpContentRangeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpContentRangeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpContentRangeHeaderValue {}
unsafe impl ::std::marker::Sync for HttpContentRangeHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpCookiePairHeaderValue(::windows::runtime::IInspectable);
impl HttpCookiePairHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, value: Param1) -> ::windows::runtime::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, cookiepairheadervalue: &mut ::std::option::Option<HttpCookiePairHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), cookiepairheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpCookiePairHeaderValueFactory<R, F: FnOnce(&IHttpCookiePairHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpCookiePairHeaderValueStatics<R, F: FnOnce(&IHttpCookiePairHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpCookiePairHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValue;{cbd46217-4b29-412b-bd90-b3d814ab8e1b})");
}
unsafe impl ::windows::runtime::Interface for HttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3419693591, 19241, 16683, [189, 144, 179, 216, 20, 171, 142, 27]);
}
impl ::windows::runtime::RuntimeName for HttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValue";
}
impl ::std::convert::From<HttpCookiePairHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpCookiePairHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpCookiePairHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpCookiePairHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCookiePairHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCookiePairHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpCookiePairHeaderValue {}
unsafe impl ::std::marker::Sync for HttpCookiePairHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpCookiePairHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpCookiePairHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpCookiePairHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpCookiePairHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpCookiePairHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpCookiePairHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpCookiePairHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpCookiePairHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpCookiePairHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpCookiePairHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection;{f3f44350-581e-4ecc-9f59-e507d04f06e6})");
}
unsafe impl ::windows::runtime::Interface for HttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4092871504, 22558, 20172, [159, 89, 229, 7, 208, 79, 6, 230]);
}
impl ::windows::runtime::RuntimeName for HttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection";
}
impl ::std::convert::From<HttpCookiePairHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpCookiePairHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpCookiePairHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpCookiePairHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpCredentialsHeaderValue(::windows::runtime::IInspectable);
impl HttpCredentialsHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Scheme(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Token(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromScheme<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0) -> ::windows::runtime::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scheme: Param0, token: Param1) -> ::windows::runtime::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), scheme.into_param().abi(), token.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, credentialsheadervalue: &mut ::std::option::Option<HttpCredentialsHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), credentialsheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpCredentialsHeaderValueFactory<R, F: FnOnce(&IHttpCredentialsHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpCredentialsHeaderValueStatics<R, F: FnOnce(&IHttpCredentialsHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpCredentialsHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCredentialsHeaderValue;{c34cc3cb-542e-4177-a6c7-b674ce193fbf})");
}
unsafe impl ::windows::runtime::Interface for HttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3276587979, 21550, 16759, [166, 199, 182, 116, 206, 25, 63, 191]);
}
impl ::windows::runtime::RuntimeName for HttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCredentialsHeaderValue";
}
impl ::std::convert::From<HttpCredentialsHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpCredentialsHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpCredentialsHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpCredentialsHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpCredentialsHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpCredentialsHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpCredentialsHeaderValue {}
unsafe impl ::std::marker::Sync for HttpCredentialsHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpDateOrDeltaHeaderValue(::windows::runtime::IInspectable);
impl HttpDateOrDeltaHeaderValue {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Date(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Delta(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpDateOrDeltaHeaderValue> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, dateordeltaheadervalue: &mut ::std::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), dateordeltaheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpDateOrDeltaHeaderValueStatics<R, F: FnOnce(&IHttpDateOrDeltaHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpDateOrDeltaHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue;{eafcaa6a-c4dc-49e2-a27d-043adf5867a3})");
}
unsafe impl ::windows::runtime::Interface for HttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3942427242, 50396, 18914, [162, 125, 4, 58, 223, 88, 103, 163]);
}
impl ::windows::runtime::RuntimeName for HttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue";
}
impl ::std::convert::From<HttpDateOrDeltaHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpDateOrDeltaHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpDateOrDeltaHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpDateOrDeltaHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpDateOrDeltaHeaderValue {}
unsafe impl ::std::marker::Sync for HttpDateOrDeltaHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpExpectationHeaderValue(::windows::runtime::IInspectable);
impl HttpExpectationHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, value: Param1) -> ::windows::runtime::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, expectationheadervalue: &mut ::std::option::Option<HttpExpectationHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), expectationheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpExpectationHeaderValueFactory<R, F: FnOnce(&IHttpExpectationHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpExpectationHeaderValueStatics<R, F: FnOnce(&IHttpExpectationHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpExpectationHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValue;{4ce585cd-3a99-43af-a2e6-ec232fea9658})");
}
unsafe impl ::windows::runtime::Interface for HttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290110413, 15001, 17327, [162, 230, 236, 35, 47, 234, 150, 88]);
}
impl ::windows::runtime::RuntimeName for HttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValue";
}
impl ::std::convert::From<HttpExpectationHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpExpectationHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpExpectationHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpExpectationHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpExpectationHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpExpectationHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpExpectationHeaderValue {}
unsafe impl ::std::marker::Sync for HttpExpectationHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpExpectationHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpExpectationHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpExpectationHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpExpectationHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpExpectationHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpExpectationHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpExpectationHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpExpectationHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpExpectationHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpExpectationHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection;{e78521b3-a0e2-4ac4-9e66-79706cb9fd58})");
}
unsafe impl ::windows::runtime::Interface for HttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3884261811, 41186, 19140, [158, 102, 121, 112, 108, 185, 253, 88]);
}
impl ::windows::runtime::RuntimeName for HttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection";
}
impl ::std::convert::From<HttpExpectationHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpExpectationHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpExpectationHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpExpectationHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpExpectationHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpExpectationHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpLanguageHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpLanguageHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Globalization::Language>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Globalization::Language>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<super::super::super::Globalization::Language> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Globalization::Language>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Globalization::Language>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::Language>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::Language>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::Language>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::Language>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::super::super::Globalization::Language as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`, `Globalization`*"]
    pub fn ReplaceAll(&self, items: &[<super::super::super::Globalization::Language as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpLanguageHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection;{9ebd7ca3-8219-44f6-9902-8c56dfd3340c})");
}
unsafe impl ::windows::runtime::Interface for HttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2663218339, 33305, 17654, [153, 2, 140, 86, 223, 211, 52, 12]);
}
impl ::windows::runtime::RuntimeName for HttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection";
}
impl ::std::convert::From<HttpLanguageHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpLanguageHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpLanguageHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpLanguageHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpLanguageHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpLanguageHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpLanguageRangeWithQualityHeaderValue(::windows::runtime::IInspectable);
impl HttpLanguageRangeWithQualityHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn LanguageRange(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Quality(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromLanguageRange<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagerange: Param0) -> ::windows::runtime::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languagerange.into_param().abi(), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromLanguageRangeWithQuality<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagerange: Param0, quality: f64) -> ::windows::runtime::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), languagerange.into_param().abi(), quality, &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, languagerangewithqualityheadervalue: &mut ::std::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), languagerangewithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpLanguageRangeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpLanguageRangeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue;{7256e102-0080-4db4-a083-7de7b2e5ba4c})");
}
unsafe impl ::windows::runtime::Interface for HttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1918296322, 128, 19892, [160, 131, 125, 231, 178, 229, 186, 76]);
}
impl ::windows::runtime::RuntimeName for HttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue";
}
impl ::std::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl ::std::marker::Sync for HttpLanguageRangeWithQualityHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpLanguageRangeWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpLanguageRangeWithQualityHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpLanguageRangeWithQualityHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpLanguageRangeWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpLanguageRangeWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection;{885d5abd-4b4f-480a-89ce-8aedcee6e3a0})");
}
unsafe impl ::windows::runtime::Interface for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2287819453, 19279, 18442, [137, 206, 138, 237, 206, 230, 227, 160]);
}
impl ::windows::runtime::RuntimeName for HttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection";
}
impl ::std::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpLanguageRangeWithQualityHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpMediaTypeHeaderValue(::windows::runtime::IInspectable);
impl HttpMediaTypeHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CharSet(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetCharSet<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn MediaType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetMediaType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(mediatype: Param0) -> ::windows::runtime::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mediatype.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, mediatypeheadervalue: &mut ::std::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), mediatypeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpMediaTypeHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpMediaTypeHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpMediaTypeHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeHeaderValue;{16b28533-e728-4fcb-bdb0-08a431a14844})");
}
unsafe impl ::windows::runtime::Interface for HttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(380798259, 59176, 20427, [189, 176, 8, 164, 49, 161, 72, 68]);
}
impl ::windows::runtime::RuntimeName for HttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeHeaderValue";
}
impl ::std::convert::From<HttpMediaTypeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpMediaTypeHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpMediaTypeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpMediaTypeHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMediaTypeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMediaTypeHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpMediaTypeHeaderValue {}
unsafe impl ::std::marker::Sync for HttpMediaTypeHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpMediaTypeWithQualityHeaderValue(::windows::runtime::IInspectable);
impl HttpMediaTypeWithQualityHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CharSet(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetCharSet<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn MediaType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetMediaType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Quality(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetQuality<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromMediaType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(mediatype: Param0) -> ::windows::runtime::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mediatype.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromMediaTypeWithQuality<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(mediatype: Param0, quality: f64) -> ::windows::runtime::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mediatype.into_param().abi(), quality, &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, mediatypewithqualityheadervalue: &mut ::std::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), mediatypewithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpMediaTypeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpMediaTypeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue;{188d5e32-76be-44a0-b1cd-2074bded2dde})");
}
unsafe impl ::windows::runtime::Interface for HttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(411917874, 30398, 17568, [177, 205, 32, 116, 189, 237, 45, 222]);
}
impl ::windows::runtime::RuntimeName for HttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue";
}
impl ::std::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl ::std::marker::Sync for HttpMediaTypeWithQualityHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpMediaTypeWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpMediaTypeWithQualityHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpMediaTypeWithQualityHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpMediaTypeWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpMediaTypeWithQualityHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection;{3c0c6b73-1342-4587-a056-18d02ff67165})");
}
unsafe impl ::windows::runtime::Interface for HttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007446899, 4930, 17799, [160, 86, 24, 208, 47, 246, 113, 101]);
}
impl ::windows::runtime::RuntimeName for HttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection";
}
impl ::std::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpMediaTypeWithQualityHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpMethodHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpMethodHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::HttpMethod>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::HttpMethod>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<super::HttpMethod> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<super::HttpMethod>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::HttpMethod>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::HttpMethod>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, super::HttpMethod>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, super::HttpMethod>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::HttpMethod as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<super::HttpMethod as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpMethodHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMethodHeaderValueCollection;{43bc3ff4-6119-4adf-938c-34bfffcf92ed})");
}
unsafe impl ::windows::runtime::Interface for HttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1136410612, 24857, 19167, [147, 140, 52, 191, 255, 207, 146, 237]);
}
impl ::windows::runtime::RuntimeName for HttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMethodHeaderValueCollection";
}
impl ::std::convert::From<HttpMethodHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpMethodHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpMethodHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpMethodHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpMethodHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpMethodHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpNameValueHeaderValue(::windows::runtime::IInspectable);
impl HttpNameValueHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, value: Param1) -> ::windows::runtime::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, namevalueheadervalue: &mut ::std::option::Option<HttpNameValueHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), namevalueheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpNameValueHeaderValueFactory<R, F: FnOnce(&IHttpNameValueHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpNameValueHeaderValueStatics<R, F: FnOnce(&IHttpNameValueHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpNameValueHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpNameValueHeaderValue;{d8ba7463-5b9a-4d1b-93f9-aa5b44ecfddf})");
}
unsafe impl ::windows::runtime::Interface for HttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636098147, 23450, 19739, [147, 249, 170, 91, 68, 236, 253, 223]);
}
impl ::windows::runtime::RuntimeName for HttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpNameValueHeaderValue";
}
impl ::std::convert::From<HttpNameValueHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpNameValueHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpNameValueHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpNameValueHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpNameValueHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpNameValueHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpNameValueHeaderValue {}
unsafe impl ::std::marker::Sync for HttpNameValueHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpProductHeaderValue(::windows::runtime::IInspectable);
impl HttpProductHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(productname: Param0) -> ::windows::runtime::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), productname.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows::runtime::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), productname.into_param().abi(), productversion.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, productheadervalue: &mut ::std::option::Option<HttpProductHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), productheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpProductHeaderValueFactory<R, F: FnOnce(&IHttpProductHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpProductHeaderValueStatics<R, F: FnOnce(&IHttpProductHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpProductHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductHeaderValue;{f4feee03-ebd4-4160-b9ff-807c5183b6e6})");
}
unsafe impl ::windows::runtime::Interface for HttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4110347779, 60372, 16736, [185, 255, 128, 124, 81, 131, 182, 230]);
}
impl ::windows::runtime::RuntimeName for HttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductHeaderValue";
}
impl ::std::convert::From<HttpProductHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpProductHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpProductHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpProductHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpProductHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpProductHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpProductHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpProductHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpProductHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpProductHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpProductHeaderValue {}
unsafe impl ::std::marker::Sync for HttpProductHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpProductInfoHeaderValue(::windows::runtime::IInspectable);
impl HttpProductInfoHeaderValue {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Product(&self) -> ::windows::runtime::Result<HttpProductHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(productcomment: Param0) -> ::windows::runtime::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), productcomment.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows::runtime::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), productname.into_param().abi(), productversion.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, productinfoheadervalue: &mut ::std::option::Option<HttpProductInfoHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), productinfoheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpProductInfoHeaderValueFactory<R, F: FnOnce(&IHttpProductInfoHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpProductInfoHeaderValueStatics<R, F: FnOnce(&IHttpProductInfoHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpProductInfoHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValue;{1b1a8732-4c35-486a-966f-646489198e4d})");
}
unsafe impl ::windows::runtime::Interface for HttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(454723378, 19509, 18538, [150, 111, 100, 100, 137, 25, 142, 77]);
}
impl ::windows::runtime::RuntimeName for HttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValue";
}
impl ::std::convert::From<HttpProductInfoHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpProductInfoHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpProductInfoHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpProductInfoHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpProductInfoHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpProductInfoHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpProductInfoHeaderValue {}
unsafe impl ::std::marker::Sync for HttpProductInfoHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpProductInfoHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpProductInfoHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpProductInfoHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpProductInfoHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpProductInfoHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpProductInfoHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpProductInfoHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpProductInfoHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpProductInfoHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpProductInfoHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection;{877df74a-d69b-44f8-ad4f-453af9c42ed0})");
}
unsafe impl ::windows::runtime::Interface for HttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2273179466, 54939, 17656, [173, 79, 69, 58, 249, 196, 46, 208]);
}
impl ::windows::runtime::RuntimeName for HttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection";
}
impl ::std::convert::From<HttpProductInfoHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpProductInfoHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpProductInfoHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpProductInfoHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpRequestHeaderCollection(::windows::runtime::IInspectable);
impl HttpRequestHeaderCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Accept(&self) -> ::windows::runtime::Result<HttpMediaTypeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn AcceptEncoding(&self) -> ::windows::runtime::Result<HttpContentCodingWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn AcceptLanguage(&self) -> ::windows::runtime::Result<HttpLanguageRangeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Authorization(&self) -> ::windows::runtime::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetAuthorization<'a, Param0: ::windows::runtime::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CacheControl(&self) -> ::windows::runtime::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Cookie(&self) -> ::windows::runtime::Result<HttpCookiePairHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpCookiePairHeaderValueCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Date(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Expect(&self) -> ::windows::runtime::Result<HttpExpectationHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpExpectationHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetFrom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Networking")]
    #[doc = "*Required features: `Web_Http_Headers`, `Networking`*"]
    pub fn Host(&self) -> ::windows::runtime::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    #[doc = "*Required features: `Web_Http_Headers`, `Networking`*"]
    pub fn SetHost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Networking::HostName>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn IfModifiedSince(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetIfModifiedSince<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn IfUnmodifiedSince(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetIfUnmodifiedSince<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn MaxForwards(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetMaxForwards<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ProxyAuthorization(&self) -> ::windows::runtime::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetProxyAuthorization<'a, Param0: ::windows::runtime::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Referer(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetReferer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TransferEncoding(&self) -> ::windows::runtime::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn UserAgent(&self) -> ::windows::runtime::Result<HttpProductInfoHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpProductInfoHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpRequestHeaderCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpRequestHeaderCollection;{af40329b-b544-469b-86b9-ac3d466fea36})");
}
unsafe impl ::windows::runtime::Interface for HttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2940220059, 46404, 18075, [134, 185, 172, 61, 70, 111, 234, 54]);
}
impl ::windows::runtime::RuntimeName for HttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpRequestHeaderCollection";
}
impl ::std::convert::From<HttpRequestHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpRequestHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpRequestHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpRequestHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpRequestHeaderCollection {}
unsafe impl ::std::marker::Sync for HttpRequestHeaderCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpResponseHeaderCollection(::windows::runtime::IInspectable);
impl HttpResponseHeaderCollection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Age(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetAge<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Allow(&self) -> ::windows::runtime::Result<HttpMethodHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethodHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn CacheControl(&self) -> ::windows::runtime::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Date(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn Location(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn SetLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ProxyAuthenticate(&self) -> ::windows::runtime::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn RetryAfter(&self) -> ::windows::runtime::Result<HttpDateOrDeltaHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn SetRetryAfter<'a, Param0: ::windows::runtime::IntoParam<'a, HttpDateOrDeltaHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TransferEncoding(&self) -> ::windows::runtime::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn WwwAuthenticate(&self) -> ::windows::runtime::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpResponseHeaderCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpResponseHeaderCollection;{7a990969-fa3f-41ed-aac6-bf957975c16b})");
}
unsafe impl ::windows::runtime::Interface for HttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2056849769, 64063, 16877, [170, 198, 191, 149, 121, 117, 193, 107]);
}
impl ::windows::runtime::RuntimeName for HttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpResponseHeaderCollection";
}
impl ::std::convert::From<HttpResponseHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpResponseHeaderCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpResponseHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpResponseHeaderCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpResponseHeaderCollection {}
unsafe impl ::std::marker::Sync for HttpResponseHeaderCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpTransferCodingHeaderValue(::windows::runtime::IInspectable);
impl HttpTransferCodingHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Parameters(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn Parse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0) -> ::windows::runtime::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, transfercodingheadervalue: &mut ::std::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::runtime::Result<bool> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), transfercodingheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IHttpTransferCodingHeaderValueFactory<R, F: FnOnce(&IHttpTransferCodingHeaderValueFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpTransferCodingHeaderValueStatics<R, F: FnOnce(&IHttpTransferCodingHeaderValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpTransferCodingHeaderValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValue;{436f32f9-3ded-42bd-b38a-5496a2511ce6})");
}
unsafe impl ::windows::runtime::Interface for HttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1131361017, 15853, 17085, [179, 138, 84, 150, 162, 81, 28, 230]);
}
impl ::windows::runtime::RuntimeName for HttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValue";
}
impl ::std::convert::From<HttpTransferCodingHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpTransferCodingHeaderValue> for ::windows::runtime::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpTransferCodingHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpTransferCodingHeaderValue> for ::windows::runtime::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpTransferCodingHeaderValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpTransferCodingHeaderValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpTransferCodingHeaderValue {}
unsafe impl ::std::marker::Sync for HttpTransferCodingHeaderValue {}
#[doc = "*Required features: `Web_Http_Headers`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HttpTransferCodingHeaderValueCollection(::windows::runtime::IInspectable);
impl HttpTransferCodingHeaderValueCollection {
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn ParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Web_Http_Headers`*"]
    pub fn TryParseAdd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<HttpTransferCodingHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<HttpTransferCodingHeaderValue> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpTransferCodingHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Web_Http_Headers`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<HttpTransferCodingHeaderValue as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute(items.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HttpTransferCodingHeaderValueCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection;{202c8c34-2c03-49b8-9665-73e27cb2fc79})");
}
unsafe impl ::windows::runtime::Interface for HttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(539790388, 11267, 18872, [150, 101, 115, 226, 124, 178, 252, 121]);
}
impl ::windows::runtime::RuntimeName for HttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection";
}
impl ::std::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HttpTransferCodingHeaderValueCollection {}
unsafe impl ::std::marker::Sync for HttpTransferCodingHeaderValueCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2589485961, 54736, 20414, [189, 157, 181, 179, 99, 104, 17, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(959668655, 3965, 18464, [159, 221, 162, 185, 86, 238, 174, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3399376769, 44768, 17235, [161, 11, 230, 37, 186, 189, 100, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpChallengeHeaderValueFactory {
    type Vtable = IHttpChallengeHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3293758545, 55708, 16554, [147, 153, 144, 238, 185, 143, 198, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpChallengeHeaderValueStatics {
    type Vtable = IHttpChallengeHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4090727026, 64513, 19713, [160, 8, 252, 183, 196, 89, 214, 53]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, challengeheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3410686586, 20112, 17899, [141, 205, 253, 20, 8, 244, 196, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3841289245, 20802, 19968, [142, 15, 1, 149, 9, 51, 118, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpConnectionOptionHeaderValueFactory {
    type Vtable = IHttpConnectionOptionHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3644640286, 2941, 19519, [165, 141, 162, 161, 189, 234, 188, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpConnectionOptionHeaderValueStatics {
    type Vtable = IHttpConnectionOptionHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2863095095, 43334, 19231, [133, 175, 72, 182, 139, 60, 80, 189]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, connectionoptionheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3170367786, 37750, 19845, [188, 204, 159, 79, 154, 202, 180, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2099386145, 42715, 17262, [142, 131, 145, 89, 97, 146, 129, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingHeaderValueFactory {
    type Vtable = IHttpContentCodingHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309120471, 13099, 17232, [133, 16, 46, 103, 162, 40, 154, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentcoding: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingHeaderValueStatics {
    type Vtable = IHttpContentCodingHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2497208366, 63935, 17143, [170, 70, 237, 39, 42, 65, 226, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contentcodingheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488474837, 35603, 19827, [134, 81, 247, 107, 56, 248, 132, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2081256766, 59545, 17272, [181, 200, 65, 45, 130, 7, 17, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingWithQualityHeaderValueFactory {
    type Vtable = IHttpContentCodingWithQualityHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3294555674, 50515, 18172, [173, 226, 215, 92, 29, 83, 223, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentcoding: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentcoding: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, quality: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentCodingWithQualityHeaderValueStatics {
    type Vtable = IHttpContentCodingWithQualityHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3905500540, 36745, 18433, [142, 117, 76, 154, 191, 195, 222, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contentcodingwithqualityheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4070764252, 9769, 19273, [153, 8, 150, 161, 104, 233, 54, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentDispositionHeaderValueFactory {
    type Vtable = IHttpContentDispositionHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2568338372, 17772, 20097, [130, 149, 178, 171, 60, 188, 245, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispositiontype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentDispositionHeaderValueStatics {
    type Vtable = IHttpContentDispositionHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(700801127, 23095, 18148, [176, 116, 197, 23, 125, 105, 202, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contentdispositionheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1080109636, 18350, 19326, [145, 36, 105, 98, 139, 100, 170, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(81356755, 42230, 18780, [149, 48, 133, 121, 252, 186, 138, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentRangeHeaderValueFactory {
    type Vtable = IHttpContentRangeHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1062983313, 41020, 17494, [154, 111, 239, 39, 236, 208, 60, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, length: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, from: u64, to: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, from: u64, to: u64, length: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpContentRangeHeaderValueStatics {
    type Vtable = IHttpContentRangeHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2158184138, 5964, 20398, [130, 28, 19, 76, 210, 148, 170, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, contentrangeheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3419693591, 19241, 16683, [189, 144, 179, 216, 20, 171, 142, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4092871504, 22558, 20172, [159, 89, 229, 7, 208, 79, 6, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCookiePairHeaderValueFactory {
    type Vtable = IHttpCookiePairHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1667117679, 5231, 20310, [170, 33, 44, 183, 214, 213, 139, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCookiePairHeaderValueStatics {
    type Vtable = IHttpCookiePairHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1854303560, 1711, 17506, [129, 88, 153, 56, 141, 93, 202, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, cookiepairheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3276587979, 21550, 16759, [166, 199, 182, 116, 206, 25, 63, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCredentialsHeaderValueFactory {
    type Vtable = IHttpCredentialsHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4062027409, 19740, 16770, [191, 209, 52, 71, 10, 98, 249, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheme: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpCredentialsHeaderValueStatics {
    type Vtable = IHttpCredentialsHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2795187174, 52876, 17475, [163, 90, 27, 114, 123, 19, 16, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, credentialsheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3942427242, 50396, 18914, [162, 125, 4, 58, 223, 88, 103, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpDateOrDeltaHeaderValueStatics {
    type Vtable = IHttpDateOrDeltaHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2082888104, 26226, 20112, [154, 154, 243, 151, 102, 247, 245, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, dateordeltaheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290110413, 15001, 17327, [162, 230, 236, 35, 47, 234, 150, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3884261811, 41186, 19140, [158, 102, 121, 112, 108, 185, 253, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpExpectationHeaderValueFactory {
    type Vtable = IHttpExpectationHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1319269835, 54590, 18536, [136, 86, 30, 33, 165, 3, 13, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpExpectationHeaderValueStatics {
    type Vtable = IHttpExpectationHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(806988770, 53221, 18235, [165, 127, 251, 165, 177, 78, 178, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, expectationheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2663218339, 33305, 17654, [153, 2, 140, 86, 223, 211, 52, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1918296322, 128, 19892, [160, 131, 125, 231, 178, 229, 186, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2287819453, 19279, 18442, [137, 206, 138, 237, 206, 230, 227, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpLanguageRangeWithQualityHeaderValueFactory {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2075670896, 30735, 19587, [159, 228, 220, 48, 135, 246, 189, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagerange: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagerange: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, quality: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpLanguageRangeWithQualityHeaderValueStatics {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(625074502, 62216, 18165, [182, 149, 66, 245, 64, 36, 236, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, languagerangewithqualityheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(380798259, 59176, 20427, [189, 176, 8, 164, 49, 161, 72, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeHeaderValueFactory {
    type Vtable = IHttpMediaTypeHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3201779624, 52503, 17117, [147, 103, 171, 156, 91, 86, 221, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeHeaderValueStatics {
    type Vtable = IHttpMediaTypeHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3763176415, 7489, 19852, [162, 222, 111, 210, 237, 135, 57, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, mediatypeheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(411917874, 30398, 17568, [177, 205, 32, 116, 189, 237, 45, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007446899, 4930, 17799, [160, 86, 24, 208, 47, 246, 113, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeWithQualityHeaderValueFactory {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1282220276, 37975, 17638, [163, 35, 209, 34, 185, 88, 120, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, quality: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMediaTypeWithQualityHeaderValueStatics {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1527188697, 46432, 20424, [152, 53, 126, 108, 10, 101, 123, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, mediatypewithqualityheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1136410612, 24857, 19167, [147, 140, 52, 191, 255, 207, 146, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636098147, 23450, 19739, [147, 249, 170, 91, 68, 236, 253, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpNameValueHeaderValueFactory {
    type Vtable = IHttpNameValueHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1997415015, 52216, 18230, [169, 37, 147, 251, 225, 12, 124, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpNameValueHeaderValueStatics {
    type Vtable = IHttpNameValueHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4292084495, 4400, 16722, [134, 89, 37, 105, 9, 169, 209, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, namevalueheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4110347779, 60372, 16736, [185, 255, 128, 124, 81, 131, 182, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductHeaderValueFactory {
    type Vtable = IHttpProductHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1629136117, 33468, 17147, [151, 123, 220, 0, 83, 110, 94, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, productversion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductHeaderValueStatics {
    type Vtable = IHttpProductHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2428714537, 48892, 17207, [190, 98, 73, 240, 151, 151, 95, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, productheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(454723378, 19509, 18538, [150, 111, 100, 100, 137, 25, 142, 77]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2273179466, 54939, 17656, [173, 79, 69, 58, 249, 196, 46, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductInfoHeaderValueFactory {
    type Vtable = IHttpProductInfoHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(606212030, 60094, 17508, [180, 96, 236, 1, 11, 124, 65, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productcomment: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, productversion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpProductInfoHeaderValueStatics {
    type Vtable = IHttpProductInfoHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3682588759, 12922, 20083, [129, 229, 112, 89, 163, 2, 176, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, productinfoheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2940220059, 46404, 18075, [134, 185, 172, 61, 70, 111, 234, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2056849769, 64063, 16877, [170, 198, 191, 149, 121, 117, 193, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1131361017, 15853, 17085, [179, 138, 84, 150, 162, 81, 28, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(539790388, 11267, 18872, [150, 101, 115, 226, 124, 178, 252, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpTransferCodingHeaderValueFactory {
    type Vtable = IHttpTransferCodingHeaderValueFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3143819260, 58209, 20232, [142, 79, 201, 231, 35, 222, 112, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHttpTransferCodingHeaderValueStatics {
    type Vtable = IHttpTransferCodingHeaderValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1790478634, 6808, 19762, [169, 6, 116, 112, 169, 135, 92, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, transfercodingheadervalue: *mut ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
