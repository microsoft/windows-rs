#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXsltProcessor2 {
    type Vtable = IXsltProcessor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8da45c56_97a5_44cb_a8be_27d86280c70a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXsltProcessorFactory {
    type Vtable = IXsltProcessorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274146c0_9a51_4663_bf30_0ef742146f20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessorFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Data_Xml_Xsl\"`*"]
#[repr(transparent)]
pub struct XsltProcessor(::windows::core::IUnknown);
impl XsltProcessor {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToString<'a, P0, E0>(&self, inputnode: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Dom::IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TransformToString)(::windows::core::Interface::as_raw(this), inputnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToDocument<'a, P0, E0>(&self, inputnode: P0) -> ::windows::core::Result<super::Dom::XmlDocument>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Dom::IXmlNode>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TransformToDocument)(::windows::core::Interface::as_raw(this), inputnode.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateInstance<'a, P0>(document: P0) -> ::windows::core::Result<XsltProcessor>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Dom::XmlDocument>>,
    {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), document.into().abi(), result__.as_mut_ptr()).from_abi::<XsltProcessor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XsltProcessor, IXsltProcessorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XsltProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XsltProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XsltProcessor {}
impl ::core::fmt::Debug for XsltProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XsltProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XsltProcessor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Xsl.XsltProcessor;{7b64703f-550c-48c6-a90f-93a5b964518f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
    const IID: ::windows::core::GUID = <IXsltProcessor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
impl ::core::convert::From<XsltProcessor> for ::windows::core::IUnknown {
    fn from(value: XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows::core::IUnknown {
    fn from(value: &XsltProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XsltProcessor> for &::windows::core::IUnknown {
    fn from(value: &XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XsltProcessor> for ::windows::core::IInspectable {
    fn from(value: XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows::core::IInspectable {
    fn from(value: &XsltProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XsltProcessor> for &::windows::core::IInspectable {
    fn from(value: &XsltProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XsltProcessor {}
unsafe impl ::core::marker::Sync for XsltProcessor {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
