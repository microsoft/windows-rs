#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IXsltProcessor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXsltProcessor {
    type Vtable = IXsltProcessor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputnode: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXsltProcessor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXsltProcessor2 {
    type Vtable = IXsltProcessor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8da45c56_97a5_44cb_a8be_27d86280c70a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputnode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXsltProcessorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXsltProcessorFactory {
    type Vtable = IXsltProcessorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274146c0_9a51_4663_bf30_0ef742146f20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXsltProcessorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, document: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XsltProcessor(pub ::windows::core::IInspectable);
impl XsltProcessor {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToString<'a, Param0: ::windows::core::IntoParam<'a, super::Dom::IXmlNode>>(&self, inputnode: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputnode.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn TransformToDocument<'a, Param0: ::windows::core::IntoParam<'a, super::Dom::IXmlNode>>(&self, inputnode: Param0) -> ::windows::core::Result<super::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<IXsltProcessor2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputnode.into_param().abi(), &mut result__).from_abi::<super::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::Dom::XmlDocument>>(document: Param0) -> ::windows::core::Result<XsltProcessor> {
        Self::IXsltProcessorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), document.into_param().abi(), &mut result__).from_abi::<XsltProcessor>(result__)
        })
    }
    pub fn IXsltProcessorFactory<R, F: FnOnce(&IXsltProcessorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XsltProcessor, IXsltProcessorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XsltProcessor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Xml.Xsl.XsltProcessor;{7b64703f-550c-48c6-a90f-93a5b964518f})");
}
unsafe impl ::windows::core::Interface for XsltProcessor {
    type Vtable = IXsltProcessor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b64703f_550c_48c6_a90f_93a5b964518f);
}
impl ::windows::core::RuntimeName for XsltProcessor {
    const NAME: &'static str = "Windows.Data.Xml.Xsl.XsltProcessor";
}
impl ::core::convert::From<XsltProcessor> for ::windows::core::IUnknown {
    fn from(value: XsltProcessor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows::core::IUnknown {
    fn from(value: &XsltProcessor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XsltProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XsltProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XsltProcessor> for ::windows::core::IInspectable {
    fn from(value: XsltProcessor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XsltProcessor> for ::windows::core::IInspectable {
    fn from(value: &XsltProcessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XsltProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XsltProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for XsltProcessor {}
unsafe impl ::core::marker::Sync for XsltProcessor {}
