#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IIndexedResourceCandidate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e619ef3_faec_4414_a9d7_54acd5953f29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IndexedResourceType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub ValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIndexedResourceQualifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdae3bb9b_d304_497f_a168_a340042c8adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub QualifierName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexer(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IResourceIndexer {
    type Vtable = IResourceIndexer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4cf9a5_e32f_4ab2_8748_96350a016da3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IndexFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IndexFilePath: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub IndexFileContentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    IndexFileContentsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexerFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IResourceIndexerFactory {
    type Vtable = IResourceIndexerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8de3f09_31cd_4d97_bd30_8d39f742bc61);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexer: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexerFactory2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IResourceIndexerFactory2 {
    type Vtable = IResourceIndexerFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6040f18d_d5e5_4b60_9201_cd279cbcfed9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexerWithExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, extensiondllpath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexerWithExtension: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
pub struct IndexedResourceCandidate(::windows::core::IUnknown);
impl IndexedResourceCandidate {
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<IndexedResourceType> {
        let this = self;
        unsafe {
            let mut result__: IndexedResourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IndexedResourceType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Metadata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Qualifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueAsString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
    pub fn GetQualifierValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, qualifiername: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetQualifierValue)(::core::mem::transmute_copy(this), qualifiername.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for IndexedResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IndexedResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IndexedResourceCandidate {}
impl ::core::fmt::Debug for IndexedResourceCandidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedResourceCandidate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceCandidate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate;{0e619ef3-faec-4414-a9d7-54acd5953f29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_Vtbl;
    const IID: ::windows::core::GUID = <IIndexedResourceCandidate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate";
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: IndexedResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: &IndexedResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: IndexedResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: &IndexedResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IndexedResourceCandidate {}
unsafe impl ::core::marker::Sync for IndexedResourceCandidate {}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
pub struct IndexedResourceQualifier(::windows::core::IUnknown);
impl IndexedResourceQualifier {
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
    pub fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QualifierName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
    pub fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QualifierValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for IndexedResourceQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IndexedResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IndexedResourceQualifier {}
impl ::core::fmt::Debug for IndexedResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedResourceQualifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceQualifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier;{dae3bb9b-d304-497f-a168-a340042c8adb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_Vtbl;
    const IID: ::windows::core::GUID = <IIndexedResourceQualifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier";
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: IndexedResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: &IndexedResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: IndexedResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: &IndexedResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IndexedResourceQualifier {}
unsafe impl ::core::marker::Sync for IndexedResourceQualifier {}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: Self = Self(0i32);
    pub const Path: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for IndexedResourceType {}
impl ::core::clone::Clone for IndexedResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IndexedResourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IndexedResourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for IndexedResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedResourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedResourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Management.IndexedResourceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ResourceIndexer(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ResourceIndexer {
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IndexFilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, filepath: Param0) -> ::windows::core::Result<IndexedResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexFilePath)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<IndexedResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn IndexFileContentsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, file: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexFileContentsAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateResourceIndexer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0) -> ::windows::core::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceIndexer)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateResourceIndexerWithExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0, extensiondllpath: Param1) -> ::windows::core::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateResourceIndexerWithExtension)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), extensiondllpath.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IResourceIndexerFactory<R, F: FnOnce(&IResourceIndexerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceIndexer, IResourceIndexerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IResourceIndexerFactory2<R, F: FnOnce(&IResourceIndexerFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceIndexer, IResourceIndexerFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ResourceIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ResourceIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ResourceIndexer {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ResourceIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceIndexer").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for ResourceIndexer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.ResourceIndexer;{2d4cf9a5-e32f-4ab2-8748-96350a016da3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ResourceIndexer {
    type Vtable = IResourceIndexer_Vtbl;
    const IID: ::windows::core::GUID = <IResourceIndexer as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.ResourceIndexer";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ResourceIndexer> for ::windows::core::IUnknown {
    fn from(value: ResourceIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ResourceIndexer> for ::windows::core::IUnknown {
    fn from(value: &ResourceIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<ResourceIndexer> for ::windows::core::IInspectable {
    fn from(value: ResourceIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&ResourceIndexer> for ::windows::core::IInspectable {
    fn from(value: &ResourceIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for ResourceIndexer {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for ResourceIndexer {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
