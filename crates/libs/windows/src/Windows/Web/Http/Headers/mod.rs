#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpCacheDirectiveHeaderValueCollection(::windows::core::IUnknown);
impl HttpCacheDirectiveHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAge<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxStale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxStale<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MinFresh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMinFresh<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SharedMaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSharedMaxAge<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpNameValueHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpNameValueHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpNameValueHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpNameValueHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCacheDirectiveHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCacheDirectiveHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection;{9a586b89-d5d0-4fbe-bd9d-b5b3636811b4})");
}
unsafe impl ::windows::core::Interface for HttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a586b89_d5d0_4fbe_bd9d_b5b3636811b4);
}
impl ::windows::core::RuntimeName for HttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCacheDirectiveHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpChallengeHeaderValue(::windows::core::IUnknown);
impl HttpChallengeHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromScheme<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0, token: Param1) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), scheme.into_param().abi(), token.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), challengeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpChallengeHeaderValueFactory<R, F: FnOnce(&IHttpChallengeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpChallengeHeaderValueStatics<R, F: FnOnce(&IHttpChallengeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpChallengeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValue;{393361af-0f7d-4820-9fdd-a2b956eeaeab})");
}
unsafe impl ::windows::core::Interface for HttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393361af_0f7d_4820_9fdd_a2b956eeaeab);
}
impl ::windows::core::RuntimeName for HttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValue";
}
impl ::core::convert::From<HttpChallengeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpChallengeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpChallengeHeaderValueCollection(::windows::core::IUnknown);
impl HttpChallengeHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpChallengeHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpChallengeHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpChallengeHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpChallengeHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpChallengeHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpChallengeHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpChallengeHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpChallengeHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection;{ca9e5f81-aee0-4353-a10b-e625babd64c2})");
}
unsafe impl ::windows::core::Interface for HttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca9e5f81_aee0_4353_a10b_e625babd64c2);
}
impl ::windows::core::RuntimeName for HttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpChallengeHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpChallengeHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValue(::windows::core::IUnknown);
impl HttpConnectionOptionHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(token: Param0) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), connectionoptionheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpConnectionOptionHeaderValueFactory<R, F: FnOnce(&IHttpConnectionOptionHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpConnectionOptionHeaderValueStatics<R, F: FnOnce(&IHttpConnectionOptionHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpConnectionOptionHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue;{cb4af27a-4e90-45eb-8dcd-fd1408f4c44f})");
}
unsafe impl ::windows::core::Interface for HttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb4af27a_4e90_45eb_8dcd_fd1408f4c44f);
}
impl ::windows::core::RuntimeName for HttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue";
}
impl ::core::convert::From<HttpConnectionOptionHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValueCollection(::windows::core::IUnknown);
impl HttpConnectionOptionHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpConnectionOptionHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpConnectionOptionHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection;{e4f56c1d-5142-4e00-8e0f-019509337629})");
}
unsafe impl ::windows::core::Interface for HttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4f56c1d_5142_4e00_8e0f_019509337629);
}
impl ::windows::core::RuntimeName for HttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentCodingHeaderValue(::windows::core::IUnknown);
impl HttpContentCodingHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contentcoding: Param0) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentcoding.into_param().abi(), &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), contentcodingheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValue;{bcf7f92a-9376-4d85-bccc-9f4f9acab434})");
}
unsafe impl ::windows::core::Interface for HttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf7f92a_9376_4d85_bccc_9f4f9acab434);
}
impl ::windows::core::RuntimeName for HttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValue";
}
impl ::core::convert::From<HttpContentCodingHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentCodingHeaderValueCollection(::windows::core::IUnknown);
impl HttpContentCodingHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpContentCodingHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpContentCodingHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpContentCodingHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpContentCodingHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection;{7d221721-a6db-436e-8e83-91596192819c})");
}
unsafe impl ::windows::core::Interface for HttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d221721_a6db_436e_8e83_91596192819c);
}
impl ::windows::core::RuntimeName for HttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpContentCodingWithQualityHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contentcoding: Param0) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentcoding.into_param().abi(), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromValueWithQuality<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contentcoding: Param0, quality: f64) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), contentcoding.into_param().abi(), quality, &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), contentcodingwithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue;{94531cd5-8b13-4d73-8651-f76b38f88495})");
}
unsafe impl ::windows::core::Interface for HttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94531cd5_8b13_4d73_8651_f76b38f88495);
}
impl ::windows::core::RuntimeName for HttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue";
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpContentCodingWithQualityHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpContentCodingWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpContentCodingWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection;{7c0d753e-e899-4378-b5c8-412d820711cc})");
}
unsafe impl ::windows::core::Interface for HttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c0d753e_e899_4378_b5c8_412d820711cc);
}
impl ::windows::core::RuntimeName for HttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentDispositionHeaderValue(::windows::core::IUnknown);
impl HttpContentDispositionHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn DispositionType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetDispositionType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn FileNameStar(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetFileNameStar<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(dispositiontype: Param0) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dispositiontype.into_param().abi(), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), contentdispositionheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentDispositionHeaderValueFactory<R, F: FnOnce(&IHttpContentDispositionHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpContentDispositionHeaderValueStatics<R, F: FnOnce(&IHttpContentDispositionHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentDispositionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentDispositionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentDispositionHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpContentDispositionHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentDispositionHeaderValue;{f2a2eedc-2629-4b49-9908-96a168e9365e})");
}
unsafe impl ::windows::core::Interface for HttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2a2eedc_2629_4b49_9908_96a168e9365e);
}
impl ::windows::core::RuntimeName for HttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentDispositionHeaderValue";
}
impl ::core::convert::From<HttpContentDispositionHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentDispositionHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentDispositionHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentDispositionHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentDispositionHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentDispositionHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentDispositionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentDispositionHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentHeaderCollection(::windows::core::IUnknown);
impl HttpContentHeaderCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentHeaderCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentDisposition(&self) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentDispositionHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetContentDisposition<'a, Param0: ::windows::core::IntoParam<'a, HttpContentDispositionHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentEncoding(&self) -> ::windows::core::Result<HttpContentCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentLanguage(&self) -> ::windows::core::Result<HttpLanguageHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpLanguageHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLocation(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentMD5(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContentMD5<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentRange(&self) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetContentRange<'a, Param0: ::windows::core::IntoParam<'a, HttpContentRangeHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ContentType(&self) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, HttpMediaTypeHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Expires(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpires<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LastModified(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastModified<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpContentHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentHeaderCollection {}
unsafe impl ::windows::core::RuntimeType for HttpContentHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentHeaderCollection;{40612a44-47ae-4b7e-9124-69628b64aa18})");
}
unsafe impl ::windows::core::Interface for HttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40612a44_47ae_4b7e_9124_69628b64aa18);
}
impl ::windows::core::RuntimeName for HttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpContentHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: HttpContentHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: HttpContentHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpContentHeaderCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpContentRangeHeaderValue(::windows::core::IUnknown);
impl HttpContentRangeHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FirstBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LastBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Length(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Unit(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetUnit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromLength(length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromRange(from: u64, to: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), from, to, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromRangeWithLength(from: u64, to: u64, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), from, to, length, &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), contentrangeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentRangeHeaderValueFactory<R, F: FnOnce(&IHttpContentRangeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpContentRangeHeaderValueStatics<R, F: FnOnce(&IHttpContentRangeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentRangeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentRangeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentRangeHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpContentRangeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentRangeHeaderValue;{04d967d3-a4f6-495c-9530-8579fcba8aa9})");
}
unsafe impl ::windows::core::Interface for HttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d967d3_a4f6_495c_9530_8579fcba8aa9);
}
impl ::windows::core::RuntimeName for HttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentRangeHeaderValue";
}
impl ::core::convert::From<HttpContentRangeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentRangeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentRangeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentRangeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentRangeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentRangeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentRangeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentRangeHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpCookiePairHeaderValue(::windows::core::IUnknown);
impl HttpCookiePairHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, value: Param1) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), cookiepairheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCookiePairHeaderValueFactory<R, F: FnOnce(&IHttpCookiePairHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpCookiePairHeaderValueStatics<R, F: FnOnce(&IHttpCookiePairHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpCookiePairHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValue;{cbd46217-4b29-412b-bd90-b3d814ab8e1b})");
}
unsafe impl ::windows::core::Interface for HttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd46217_4b29_412b_bd90_b3d814ab8e1b);
}
impl ::windows::core::RuntimeName for HttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValue";
}
impl ::core::convert::From<HttpCookiePairHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpCookiePairHeaderValueCollection(::windows::core::IUnknown);
impl HttpCookiePairHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpCookiePairHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpCookiePairHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpCookiePairHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpCookiePairHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpCookiePairHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpCookiePairHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpCookiePairHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpCookiePairHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection;{f3f44350-581e-4ecc-9f59-e507d04f06e6})");
}
unsafe impl ::windows::core::Interface for HttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f44350_581e_4ecc_9f59_e507d04f06e6);
}
impl ::windows::core::RuntimeName for HttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpCredentialsHeaderValue(::windows::core::IUnknown);
impl HttpCredentialsHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromScheme<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scheme.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scheme: Param0, token: Param1) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), scheme.into_param().abi(), token.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), credentialsheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCredentialsHeaderValueFactory<R, F: FnOnce(&IHttpCredentialsHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpCredentialsHeaderValueStatics<R, F: FnOnce(&IHttpCredentialsHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCredentialsHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCredentialsHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCredentialsHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpCredentialsHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCredentialsHeaderValue;{c34cc3cb-542e-4177-a6c7-b674ce193fbf})");
}
unsafe impl ::windows::core::Interface for HttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc34cc3cb_542e_4177_a6c7_b674ce193fbf);
}
impl ::windows::core::RuntimeName for HttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCredentialsHeaderValue";
}
impl ::core::convert::From<HttpCredentialsHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCredentialsHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCredentialsHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCredentialsHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCredentialsHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCredentialsHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCredentialsHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCredentialsHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpDateOrDeltaHeaderValue(::windows::core::IUnknown);
impl HttpDateOrDeltaHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), dateordeltaheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpDateOrDeltaHeaderValueStatics<R, F: FnOnce(&IHttpDateOrDeltaHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpDateOrDeltaHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDateOrDeltaHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDateOrDeltaHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpDateOrDeltaHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue;{eafcaa6a-c4dc-49e2-a27d-043adf5867a3})");
}
unsafe impl ::windows::core::Interface for HttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeafcaa6a_c4dc_49e2_a27d_043adf5867a3);
}
impl ::windows::core::RuntimeName for HttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue";
}
impl ::core::convert::From<HttpDateOrDeltaHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDateOrDeltaHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpDateOrDeltaHeaderValue {}
unsafe impl ::core::marker::Sync for HttpDateOrDeltaHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpExpectationHeaderValue(::windows::core::IUnknown);
impl HttpExpectationHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, value: Param1) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), expectationheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpExpectationHeaderValueFactory<R, F: FnOnce(&IHttpExpectationHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpExpectationHeaderValueStatics<R, F: FnOnce(&IHttpExpectationHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpExpectationHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValue;{4ce585cd-3a99-43af-a2e6-ec232fea9658})");
}
unsafe impl ::windows::core::Interface for HttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce585cd_3a99_43af_a2e6_ec232fea9658);
}
impl ::windows::core::RuntimeName for HttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValue";
}
impl ::core::convert::From<HttpExpectationHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpExpectationHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValue {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpExpectationHeaderValueCollection(::windows::core::IUnknown);
impl HttpExpectationHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpExpectationHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpExpectationHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpExpectationHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpExpectationHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpExpectationHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpExpectationHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpExpectationHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpExpectationHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection;{e78521b3-a0e2-4ac4-9e66-79706cb9fd58})");
}
unsafe impl ::windows::core::Interface for HttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe78521b3_a0e2_4ac4_9e66_79706cb9fd58);
}
impl ::windows::core::RuntimeName for HttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpExpectationHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpExpectationHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpLanguageHeaderValueCollection(::windows::core::IUnknown);
impl HttpLanguageHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Globalization::Language>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Globalization::Language>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Globalization::Language>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Globalization::Language>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::Language>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::Language>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::Language>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::Language>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<super::super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpLanguageHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection;{9ebd7ca3-8219-44f6-9902-8c56dfd3340c})");
}
unsafe impl ::windows::core::Interface for HttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ebd7ca3_8219_44f6_9902_8c56dfd3340c);
}
impl ::windows::core::RuntimeName for HttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpLanguageHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn LanguageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromLanguageRange<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagerange: Param0) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagerange.into_param().abi(), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromLanguageRangeWithQuality<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagerange: Param0, quality: f64) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), languagerange.into_param().abi(), quality, &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), languagerangewithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpLanguageRangeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpLanguageRangeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue;{7256e102-0080-4db4-a083-7de7b2e5ba4c})");
}
unsafe impl ::windows::core::Interface for HttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7256e102_0080_4db4_a083_7de7b2e5ba4c);
}
impl ::windows::core::RuntimeName for HttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue";
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpLanguageRangeWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpLanguageRangeWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection;{885d5abd-4b4f-480a-89ce-8aedcee6e3a0})");
}
unsafe impl ::windows::core::Interface for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x885d5abd_4b4f_480a_89ce_8aedcee6e3a0);
}
impl ::windows::core::RuntimeName for HttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpMediaTypeHeaderValue(::windows::core::IUnknown);
impl HttpMediaTypeHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetCharSet<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetMediaType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(mediatype: Param0) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediatype.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), mediatypeheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMediaTypeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeHeaderValue;{16b28533-e728-4fcb-bdb0-08a431a14844})");
}
unsafe impl ::windows::core::Interface for HttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16b28533_e728_4fcb_bdb0_08a431a14844);
}
impl ::windows::core::RuntimeName for HttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeHeaderValue";
}
impl ::core::convert::From<HttpMediaTypeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetCharSet<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetMediaType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetQuality<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromMediaType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(mediatype: Param0) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediatype.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromMediaTypeWithQuality<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(mediatype: Param0, quality: f64) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediatype.into_param().abi(), quality, &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), mediatypewithqualityheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue;{188d5e32-76be-44a0-b1cd-2074bded2dde})");
}
unsafe impl ::windows::core::Interface for HttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x188d5e32_76be_44a0_b1cd_2074bded2dde);
}
impl ::windows::core::RuntimeName for HttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue";
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpMediaTypeWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpMediaTypeWithQualityHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection;{3c0c6b73-1342-4587-a056-18d02ff67165})");
}
unsafe impl ::windows::core::Interface for HttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0c6b73_1342_4587_a056_18d02ff67165);
}
impl ::windows::core::RuntimeName for HttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpMethodHeaderValueCollection(::windows::core::IUnknown);
impl HttpMethodHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::HttpMethod>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::HttpMethod>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::HttpMethod> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<super::HttpMethod>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::HttpMethod>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::HttpMethod>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, super::HttpMethod>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::HttpMethod>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<super::HttpMethod as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<super::HttpMethod as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMethodHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMethodHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethodHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpMethodHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMethodHeaderValueCollection;{43bc3ff4-6119-4adf-938c-34bfffcf92ed})");
}
unsafe impl ::windows::core::Interface for HttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bc3ff4_6119_4adf_938c_34bfffcf92ed);
}
impl ::windows::core::RuntimeName for HttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMethodHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpMethodHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethodHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMethodHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethodHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<super::HttpMethod>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMethodHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMethodHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpNameValueHeaderValue(::windows::core::IUnknown);
impl HttpNameValueHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromNameWithValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, value: Param1) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), namevalueheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpNameValueHeaderValueFactory<R, F: FnOnce(&IHttpNameValueHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpNameValueHeaderValueStatics<R, F: FnOnce(&IHttpNameValueHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpNameValueHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpNameValueHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpNameValueHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpNameValueHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpNameValueHeaderValue;{d8ba7463-5b9a-4d1b-93f9-aa5b44ecfddf})");
}
unsafe impl ::windows::core::Interface for HttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ba7463_5b9a_4d1b_93f9_aa5b44ecfddf);
}
impl ::windows::core::RuntimeName for HttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpNameValueHeaderValue";
}
impl ::core::convert::From<HttpNameValueHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpNameValueHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpNameValueHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpNameValueHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpNameValueHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpNameValueHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpNameValueHeaderValue {}
unsafe impl ::core::marker::Sync for HttpNameValueHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpProductHeaderValue(::windows::core::IUnknown);
impl HttpProductHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(productname: Param0) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), productname.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), productname.into_param().abi(), productversion.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), productheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpProductHeaderValueFactory<R, F: FnOnce(&IHttpProductHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpProductHeaderValueStatics<R, F: FnOnce(&IHttpProductHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpProductHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpProductHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductHeaderValue;{f4feee03-ebd4-4160-b9ff-807c5183b6e6})");
}
unsafe impl ::windows::core::Interface for HttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4feee03_ebd4_4160_b9ff_807c5183b6e6);
}
impl ::windows::core::RuntimeName for HttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductHeaderValue";
}
impl ::core::convert::From<HttpProductHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpProductHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpProductHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpProductHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpProductHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpProductInfoHeaderValue(::windows::core::IUnknown);
impl HttpProductInfoHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Product(&self) -> ::windows::core::Result<HttpProductHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpProductHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(productcomment: Param0) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), productcomment.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), productname.into_param().abi(), productversion.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), productinfoheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpProductInfoHeaderValueFactory<R, F: FnOnce(&IHttpProductInfoHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpProductInfoHeaderValueStatics<R, F: FnOnce(&IHttpProductInfoHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpProductInfoHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValue;{1b1a8732-4c35-486a-966f-646489198e4d})");
}
unsafe impl ::windows::core::Interface for HttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1a8732_4c35_486a_966f_646489198e4d);
}
impl ::windows::core::RuntimeName for HttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValue";
}
impl ::core::convert::From<HttpProductInfoHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpProductInfoHeaderValueCollection(::windows::core::IUnknown);
impl HttpProductInfoHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpProductInfoHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpProductInfoHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpProductInfoHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpProductInfoHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpProductInfoHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpProductInfoHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpProductInfoHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpProductInfoHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection;{877df74a-d69b-44f8-ad4f-453af9c42ed0})");
}
unsafe impl ::windows::core::Interface for HttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877df74a_d69b_44f8_ad4f_453af9c42ed0);
}
impl ::windows::core::RuntimeName for HttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValueCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpRequestHeaderCollection(::windows::core::IUnknown);
impl HttpRequestHeaderCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Accept(&self) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMediaTypeWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn AcceptEncoding(&self) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpContentCodingWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn AcceptLanguage(&self) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpLanguageRangeWithQualityHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Authorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetAuthorization<'a, Param0: ::windows::core::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Cookie(&self) -> ::windows::core::Result<HttpCookiePairHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCookiePairHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Expect(&self) -> ::windows::core::Result<HttpExpectationHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpExpectationHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Networking'*"]
    #[cfg(feature = "Networking")]
    pub fn Host(&self) -> ::windows::core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Networking'*"]
    #[cfg(feature = "Networking")]
    pub fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IfModifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIfModifiedSince<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IfUnmodifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIfUnmodifiedSince<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxForwards(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxForwards<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ProxyAuthorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetProxyAuthorization<'a, Param0: ::windows::core::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Referer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReferer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn UserAgent(&self) -> ::windows::core::Result<HttpProductInfoHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpProductInfoHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpRequestHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestHeaderCollection {}
unsafe impl ::windows::core::RuntimeType for HttpRequestHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpRequestHeaderCollection;{af40329b-b544-469b-86b9-ac3d466fea36})");
}
unsafe impl ::windows::core::Interface for HttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf40329b_b544_469b_86b9_ac3d466fea36);
}
impl ::windows::core::RuntimeName for HttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpRequestHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpRequestHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpRequestHeaderCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpResponseHeaderCollection(::windows::core::IUnknown);
impl HttpResponseHeaderCollection {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Age(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAge<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Allow(&self) -> ::windows::core::Result<HttpMethodHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethodHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ProxyAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn RetryAfter(&self) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn SetRetryAfter<'a, Param0: ::windows::core::IntoParam<'a, HttpDateOrDeltaHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn WwwAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpResponseHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpResponseHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseHeaderCollection {}
unsafe impl ::windows::core::RuntimeType for HttpResponseHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpResponseHeaderCollection;{7a990969-fa3f-41ed-aac6-bf957975c16b})");
}
unsafe impl ::windows::core::Interface for HttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a990969_fa3f_41ed_aac6_bf957975c16b);
}
impl ::windows::core::RuntimeName for HttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpResponseHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpResponseHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseHeaderCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpResponseHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseHeaderCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpResponseHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpResponseHeaderCollection {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValue(::windows::core::IUnknown);
impl HttpTransferCodingHeaderValue {
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(input: Param0, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), transfercodingheadervalue as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpTransferCodingHeaderValueFactory<R, F: FnOnce(&IHttpTransferCodingHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpTransferCodingHeaderValueStatics<R, F: FnOnce(&IHttpTransferCodingHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValue {}
unsafe impl ::windows::core::RuntimeType for HttpTransferCodingHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValue;{436f32f9-3ded-42bd-b38a-5496a2511ce6})");
}
unsafe impl ::windows::core::Interface for HttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x436f32f9_3ded_42bd_b38a_5496a2511ce6);
}
impl ::windows::core::RuntimeName for HttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValue";
}
impl ::core::convert::From<HttpTransferCodingHeaderValue> for ::windows::core::IUnknown {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValue> for ::windows::core::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValue> for ::windows::core::IInspectable {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValue> for ::windows::core::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValue {}
#[doc = "*Required features: 'Web_Http_Headers'*"]
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValueCollection(::windows::core::IUnknown);
impl HttpTransferCodingHeaderValueCollection {
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn ParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers'*"]
    pub fn TryParseAdd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpTransferCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpTransferCodingHeaderValue>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpTransferCodingHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http_Headers', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<HttpTransferCodingHeaderValue as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValueCollection {}
unsafe impl ::windows::core::RuntimeType for HttpTransferCodingHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection;{202c8c34-2c03-49b8-9665-73e27cb2fc79})");
}
unsafe impl ::windows::core::Interface for HttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202c8c34_2c03_49b8_9665_73e27cb2fc79);
}
impl ::windows::core::RuntimeName for HttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValueCollection {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheDirectiveHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a586b89_d5d0_4fbe_bd9d_b5b3636811b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393361af_0f7d_4820_9fdd_a2b956eeaeab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca9e5f81_aee0_4353_a10b_e625babd64c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueFactory {
    type Vtable = IHttpChallengeHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc452c451_d99c_40aa_9399_90eeb98fc613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueStatics {
    type Vtable = IHttpChallengeHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3d38a72_fc01_4d01_a008_fcb7c459d635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, challengeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb4af27a_4e90_45eb_8dcd_fd1408f4c44f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4f56c1d_5142_4e00_8e0f_019509337629);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueFactory {
    type Vtable = IHttpConnectionOptionHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd93ccc1e_0b7d_4c3f_a58d_a2a1bdeabc0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueStatics {
    type Vtable = IHttpConnectionOptionHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa75d37_a946_4b1f_85af_48b68b3c50bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionoptionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf7f92a_9376_4d85_bccc_9f4f9acab434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d221721_a6db_436e_8e83_91596192819c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueFactory {
    type Vtable = IHttpContentCodingHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc53d2bd7_332b_4350_8510_2e67a2289a5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueStatics {
    type Vtable = IHttpContentCodingHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d8602e_f9bf_42f7_aa46_ed272a41e212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94531cd5_8b13_4d73_8651_f76b38f88495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c0d753e_e899_4378_b5c8_412d820711cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueFactory {
    type Vtable = IHttpContentCodingWithQualityHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45eee1a_c553_46fc_ade2_d75c1d53df7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueStatics {
    type Vtable = IHttpContentCodingWithQualityHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c9357c_8f89_4801_8e75_4c9abfc3de71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingwithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2a2eedc_2629_4b49_9908_96a168e9365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValueFactory {
    type Vtable = IHttpContentDispositionHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9915bbc4_456c_4e81_8295_b2ab3cbcf545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispositiontype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValueStatics {
    type Vtable = IHttpContentDispositionHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29c56067_5a37_46e4_b074_c5177d69ca66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentdispositionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40612a44_47ae_4b7e_9124_69628b64aa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d967d3_a4f6_495c_9530_8579fcba8aa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValueFactory {
    type Vtable = IHttpContentRangeHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5bd691_a03c_4456_9a6f_ef27ecd03cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValueStatics {
    type Vtable = IHttpContentRangeHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80a346ca_174c_4fae_821c_134cd294aa38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentrangeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd46217_4b29_412b_bd90_b3d814ab8e1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f44350_581e_4ecc_9f59_e507d04f06e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueFactory {
    type Vtable = IHttpCookiePairHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x635e326f_146f_4f56_aa21_2cb7d6d58b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueStatics {
    type Vtable = IHttpCookiePairHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e866d48_06af_4462_8158_99388d5dca81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, cookiepairheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc34cc3cb_542e_4177_a6c7_b674ce193fbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValueFactory {
    type Vtable = IHttpCredentialsHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf21d9e91_4d1c_4182_bfd1_34470a62f950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValueStatics {
    type Vtable = IHttpCredentialsHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b2be6_ce8c_4443_a35a_1b727b131036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, credentialsheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeafcaa6a_c4dc_49e2_a27d_043adf5867a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpDateOrDeltaHeaderValueStatics {
    type Vtable = IHttpDateOrDeltaHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c2659a8_6672_4e90_9a9a_f39766f7f576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dateordeltaheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce585cd_3a99_43af_a2e6_ec232fea9658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe78521b3_a0e2_4ac4_9e66_79706cb9fd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueFactory {
    type Vtable = IHttpExpectationHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ea275cb_d53e_4868_8856_1e21a5030dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueStatics {
    type Vtable = IHttpExpectationHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3019abe2_cfe5_473b_a57f_fba5b14eb257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectationheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ebd7ca3_8219_44f6_9902_8c56dfd3340c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7256e102_0080_4db4_a083_7de7b2e5ba4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x885d5abd_4b4f_480a_89ce_8aedcee6e3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueFactory {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bb83970_780f_4c83_9fe4_dc3087f6bd55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueStatics {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2541e146_f308_46f5_b695_42f54024ec68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languagerangewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16b28533_e728_4fcb_bdb0_08a431a14844);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValueFactory {
    type Vtable = IHttpMediaTypeHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed747a8_cd17_42dd_9367_ab9c5b56dd7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValueStatics {
    type Vtable = IHttpMediaTypeHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe04d83df_1d41_4d8c_a2de_6fd2ed87399b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x188d5e32_76be_44a0_b1cd_2074bded2dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0c6b73_1342_4587_a056_18d02ff67165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueFactory {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c6d20f4_9457_44e6_a323_d122b958780b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueStatics {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b070cd9_b560_4fc8_9835_7e6c0a657b24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bc3ff4_6119_4adf_938c_34bfffcf92ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ba7463_5b9a_4d1b_93f9_aa5b44ecfddf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValueFactory {
    type Vtable = IHttpNameValueHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x770e2267_cbf8_4736_a925_93fbe10c7ca8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValueStatics {
    type Vtable = IHttpNameValueHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffd4030f_1130_4152_8659_256909a9d115);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namevalueheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4feee03_ebd4_4160_b9ff_807c5183b6e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductHeaderValueFactory {
    type Vtable = IHttpProductHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x611aa4f5_82bc_42fb_977b_dc00536e5e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductHeaderValueStatics {
    type Vtable = IHttpProductHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c33e29_befc_4337_be62_49f097975f53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1a8732_4c35_486a_966f_646489198e4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877df74a_d69b_44f8_ad4f_453af9c42ed0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueFactory {
    type Vtable = IHttpProductInfoHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24220fbe_eabe_4464_b460_ec010b7c41e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productcomment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueStatics {
    type Vtable = IHttpProductInfoHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb7fd857_327a_4e73_81e5_7059a302b042);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productinfoheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf40329b_b544_469b_86b9_ac3d466fea36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a990969_fa3f_41ed_aac6_bf957975c16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x436f32f9_3ded_42bd_b38a_5496a2511ce6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202c8c34_2c03_49b8_9665_73e27cb2fc79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueFactory {
    type Vtable = IHttpTransferCodingHeaderValueFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb62dffc_e361_4f08_8e4f_c9e723de703b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueStatics {
    type Vtable = IHttpTransferCodingHeaderValueStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ab8892a_1a98_4d32_a906_7470a9875ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transfercodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
