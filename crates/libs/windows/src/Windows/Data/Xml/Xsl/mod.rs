#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
}
impl ::core::clone::Clone for IXsltProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXsltProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputnode: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXsltProcessor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXsltProcessor2 {
    type Vtable = IXsltProcessor2_Vtbl;
}
impl ::core::clone::Clone for IXsltProcessor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXsltProcessor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8da45c56_97a5_44cb_a8be_27d86280c70a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IXsltProcessorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXsltProcessorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274146c0_9a51_4663_bf30_0ef742146f20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    pub fn TransformToString<P0>(&self, inputnode: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::TryIntoParam<super::Dom::IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TransformToString)(::windows::core::Interface::as_raw(this), inputnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToDocument<P0>(&self, inputnode: P0) -> ::windows::core::Result<super::Dom::XmlDocument>
    where
        P0: ::windows::core::TryIntoParam<super::Dom::IXmlNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Dom::XmlDocument>();
            (::windows::core::Interface::vtable(this).TransformToDocument)(::windows::core::Interface::as_raw(this), inputnode.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateInstance(document: &super::Dom::XmlDocument) -> ::windows::core::Result<XsltProcessor> {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<XsltProcessor>();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(document), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<XsltProcessor, IXsltProcessorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for XsltProcessor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Xsl.XsltProcessor;{7b64703f-550c-48c6-a90f-93a5b964518f})");
}
impl ::core::clone::Clone for XsltProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XsltProcessor {
    const IID: ::windows::core::GUID = <IXsltProcessor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
::windows::imp::interface_hierarchy!(XsltProcessor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for XsltProcessor {}
unsafe impl ::core::marker::Sync for XsltProcessor {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
