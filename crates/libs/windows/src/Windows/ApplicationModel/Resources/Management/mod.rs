#[doc(hidden)]
#[repr(transparent)]
pub struct IIndexedResourceCandidate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_Vtbl;
}
impl ::core::clone::Clone for IIndexedResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIndexedResourceCandidate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e619ef3_faec_4414_a9d7_54acd5953f29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IndexedResourceType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub ValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIndexedResourceQualifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_Vtbl;
}
impl ::core::clone::Clone for IIndexedResourceQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIndexedResourceQualifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdae3bb9b_d304_497f_a168_a340042c8adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub QualifierName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexer(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IResourceIndexer {
    type Vtable = IResourceIndexer_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IResourceIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IResourceIndexer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d4cf9a5_e32f_4ab2_8748_96350a016da3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IndexFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IndexFilePath: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub IndexFileContentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    IndexFileContentsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexerFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IResourceIndexerFactory {
    type Vtable = IResourceIndexerFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IResourceIndexerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IResourceIndexerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8de3f09_31cd_4d97_bd30_8d39f742bc61);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectroot: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexer: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IResourceIndexerFactory2(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IResourceIndexerFactory2 {
    type Vtable = IResourceIndexerFactory2_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IResourceIndexerFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IResourceIndexerFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6040f18d_d5e5_4b60_9201_cd279cbcfed9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateResourceIndexerWithExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectroot: *mut ::core::ffi::c_void, extensiondllpath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateResourceIndexerWithExtension: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
pub struct IndexedResourceCandidate(::windows_core::IUnknown);
impl IndexedResourceCandidate {
    pub fn Type(&self) -> ::windows_core::Result<IndexedResourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Metadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Qualifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ValueAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetQualifierValue(&self, qualifiername: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetQualifierValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(qualifiername), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for IndexedResourceCandidate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate;{0e619ef3-faec-4414-a9d7-54acd5953f29})");
}
impl ::core::clone::Clone for IndexedResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IndexedResourceCandidate {
    const IID: ::windows_core::GUID = <IIndexedResourceCandidate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate";
}
::windows_core::imp::interface_hierarchy!(IndexedResourceCandidate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IndexedResourceCandidate {}
unsafe impl ::core::marker::Sync for IndexedResourceCandidate {}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
pub struct IndexedResourceQualifier(::windows_core::IUnknown);
impl IndexedResourceQualifier {
    pub fn QualifierName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QualifierValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for IndexedResourceQualifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier;{dae3bb9b-d304-497f-a168-a340042c8adb})");
}
impl ::core::clone::Clone for IndexedResourceQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IndexedResourceQualifier {
    const IID: ::windows_core::GUID = <IIndexedResourceQualifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier";
}
::windows_core::imp::interface_hierarchy!(IndexedResourceQualifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IndexedResourceQualifier {}
unsafe impl ::core::marker::Sync for IndexedResourceQualifier {}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ResourceIndexer(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl ResourceIndexer {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IndexFilePath<P0>(&self, filepath: P0) -> ::windows_core::Result<IndexedResourceCandidate>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexFilePath)(::windows_core::Interface::as_raw(this), filepath.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn IndexFileContentsAsync<P0>(&self, file: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexFileContentsAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateResourceIndexer<P0>(projectroot: P0) -> ::windows_core::Result<ResourceIndexer>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IResourceIndexerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceIndexer)(::windows_core::Interface::as_raw(this), projectroot.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateResourceIndexerWithExtension<P0, P1>(projectroot: P0, extensiondllpath: P1) -> ::windows_core::Result<ResourceIndexer>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IResourceIndexerFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceIndexerWithExtension)(::windows_core::Interface::as_raw(this), projectroot.into_param().abi(), extensiondllpath.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IResourceIndexerFactory<R, F: FnOnce(&IResourceIndexerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceIndexer, IResourceIndexerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IResourceIndexerFactory2<R, F: FnOnce(&IResourceIndexerFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceIndexer, IResourceIndexerFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for ResourceIndexer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.ResourceIndexer;{2d4cf9a5-e32f-4ab2-8748-96350a016da3})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ResourceIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ResourceIndexer {
    type Vtable = IResourceIndexer_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ResourceIndexer {
    const IID: ::windows_core::GUID = <IResourceIndexer as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for ResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.ResourceIndexer";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(ResourceIndexer, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for ResourceIndexer {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for ResourceIndexer {}
#[doc = "*Required features: `\"ApplicationModel_Resources_Management\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for IndexedResourceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IndexedResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedResourceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IndexedResourceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Management.IndexedResourceType;i4)");
}
