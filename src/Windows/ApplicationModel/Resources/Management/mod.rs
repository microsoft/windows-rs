#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e619ef3_faec_4414_a9d7_54acd5953f29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut IndexedResourceType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdae3bb9b_d304_497f_a168_a340042c8adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceIndexer {
    type Vtable = IResourceIndexer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4cf9a5_e32f_4ab2_8748_96350a016da3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceIndexerFactory {
    type Vtable = IResourceIndexerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8de3f09_31cd_4d97_bd30_8d39f742bc61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, projectroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceIndexerFactory2 {
    type Vtable = IResourceIndexerFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6040f18d_d5e5_4b60_9201_cd279cbcfed9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, projectroot: ::windows::core::RawPtr, extensiondllpath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IndexedResourceCandidate(pub ::windows::core::IInspectable);
impl IndexedResourceCandidate {
    pub fn Type(&self) -> ::windows::core::Result<IndexedResourceType> {
        let this = self;
        unsafe {
            let mut result__: IndexedResourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IndexedResourceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>(result__)
        }
    }
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetQualifierValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, qualifiername: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), qualifiername.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceCandidate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate;{0e619ef3-faec-4414-a9d7-54acd5953f29})");
}
unsafe impl ::windows::core::Interface for IndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e619ef3_faec_4414_a9d7_54acd5953f29);
}
impl ::windows::core::RuntimeName for IndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate";
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: IndexedResourceCandidate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: &IndexedResourceCandidate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: IndexedResourceCandidate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: &IndexedResourceCandidate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IndexedResourceCandidate {}
unsafe impl ::core::marker::Sync for IndexedResourceCandidate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IndexedResourceQualifier(pub ::windows::core::IInspectable);
impl IndexedResourceQualifier {
    pub fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceQualifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier;{dae3bb9b-d304-497f-a168-a340042c8adb})");
}
unsafe impl ::windows::core::Interface for IndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdae3bb9b_d304_497f_a168_a340042c8adb);
}
impl ::windows::core::RuntimeName for IndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier";
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: IndexedResourceQualifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: &IndexedResourceQualifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: IndexedResourceQualifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: &IndexedResourceQualifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IndexedResourceQualifier {}
unsafe impl ::core::marker::Sync for IndexedResourceQualifier {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: IndexedResourceType = IndexedResourceType(0i32);
    pub const Path: IndexedResourceType = IndexedResourceType(1i32);
    pub const EmbeddedData: IndexedResourceType = IndexedResourceType(2i32);
}
impl ::core::convert::From<i32> for IndexedResourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IndexedResourceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Management.IndexedResourceType;i4)");
}
impl ::windows::core::DefaultType for IndexedResourceType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceIndexer(pub ::windows::core::IInspectable);
impl ResourceIndexer {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn IndexFilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, filepath: Param0) -> ::windows::core::Result<IndexedResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<IndexedResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn IndexFileContentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, file: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn CreateResourceIndexer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0) -> ::windows::core::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn CreateResourceIndexerWithExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0, extensiondllpath: Param1) -> ::windows::core::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), extensiondllpath.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    pub fn IResourceIndexerFactory<R, F: FnOnce(&IResourceIndexerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceIndexer, IResourceIndexerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceIndexerFactory2<R, F: FnOnce(&IResourceIndexerFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceIndexer, IResourceIndexerFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceIndexer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.ResourceIndexer;{2d4cf9a5-e32f-4ab2-8748-96350a016da3})");
}
unsafe impl ::windows::core::Interface for ResourceIndexer {
    type Vtable = IResourceIndexer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4cf9a5_e32f_4ab2_8748_96350a016da3);
}
impl ::windows::core::RuntimeName for ResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.ResourceIndexer";
}
impl ::core::convert::From<ResourceIndexer> for ::windows::core::IUnknown {
    fn from(value: ResourceIndexer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceIndexer> for ::windows::core::IUnknown {
    fn from(value: &ResourceIndexer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceIndexer> for ::windows::core::IInspectable {
    fn from(value: ResourceIndexer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceIndexer> for ::windows::core::IInspectable {
    fn from(value: &ResourceIndexer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceIndexer {}
unsafe impl ::core::marker::Sync for ResourceIndexer {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ResourceIndexerContract(pub u8);
