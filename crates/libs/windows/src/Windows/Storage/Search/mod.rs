#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
}
impl ::core::clone::Clone for IContentIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentIndexer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1767f8d_f698_4982_b05f_3a6e8cab01a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteMultipleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteMultipleAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
}
impl ::core::clone::Clone for IContentIndexerQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentIndexerQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70e3b0f8_4bfc_428a_8889_cc51da9a7b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQuery_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesRangeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeAsync: usize,
    pub QueryFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQueryOperations(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerQueryOperations {
    type Vtable = IContentIndexerQueryOperations_Vtbl;
}
impl ::core::clone::Clone for IContentIndexerQueryOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentIndexerQueryOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28823e10_4786_42f1_9730_792b3566b150);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQueryOperations_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrderAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertiestoretrieve: *mut ::core::ffi::c_void, sortorder: *mut ::core::ffi::c_void, searchfilterlanguage: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrderAndLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertiestoretrieve: *mut ::core::ffi::c_void, sortorder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrder: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerStatics {
    type Vtable = IContentIndexerStatics_Vtbl;
}
impl ::core::clone::Clone for IContentIndexerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContentIndexerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c488375_b37e_4c60_9ba8_b760fda3e59d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetIndexerWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIndexer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IIndexableContent(::windows_core::IUnknown);
impl IIndexableContent {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StreamContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStreamContentType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamContentType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IIndexableContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IIndexableContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndexableContent {}
impl ::core::fmt::Debug for IIndexableContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndexableContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IIndexableContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4}");
}
unsafe impl ::windows_core::Interface for IIndexableContent {
    type Vtable = IIndexableContent_Vtbl;
}
impl ::core::clone::Clone for IIndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIndexableContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccf1a05f_d4b5_483a_b06e_e0db1ec420e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexableContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    pub StreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptions {
    type Vtable = IQueryOptions_Vtbl;
}
impl ::core::clone::Clone for IQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQueryOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e5e46ee_0f45_4838_a8e9_d0479d446c30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    pub FolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FolderDepth) -> ::windows_core::HRESULT,
    pub SetFolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FolderDepth) -> ::windows_core::HRESULT,
    pub ApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IndexerOption) -> ::windows_core::HRESULT,
    pub SetIndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IndexerOption) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SortOrder: usize,
    pub GroupPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DateStackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DateStackOption) -> ::windows_core::HRESULT,
    pub SaveToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LoadFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetThumbnailPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetThumbnailPrefetch: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub SetPropertyPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_FileProperties")))]
    SetPropertyPrefetch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptionsFactory {
    type Vtable = IQueryOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for IQueryOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQueryOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x032e1f8c_a9c1_4e71_8011_0dee9d4811a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCommonFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, filetypefilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCommonFileQuery: usize,
    pub CreateCommonFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsWithProviderFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptionsWithProviderFilter {
    type Vtable = IQueryOptionsWithProviderFilter_Vtbl;
}
impl ::core::clone::Clone for IQueryOptionsWithProviderFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IQueryOptionsWithProviderFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b9d1026_15c4_44dd_b89a_47a59b7d7c4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsWithProviderFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderIdFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderIdFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
}
impl ::core::clone::Clone for IStorageFileQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageFileQueryResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52fda447_2baa_412c_b29f_d4b1778efa1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFileQueryResult2 {
    type Vtable = IStorageFileQueryResult2_Vtbl;
}
impl ::core::clone::Clone for IStorageFileQueryResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageFileQueryResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e5db9dd_7141_46c4_8be3_e9dc9e27275c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub GetMatchingPropertiesWithRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    GetMatchingPropertiesWithRanges: usize,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IStorageFolderQueryOperations(::windows_core::IUnknown);
impl IStorageFolderQueryOperations {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIndexedStateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IndexedState>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexedStateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQuery)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFileQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<StorageFileQueryResult>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQuery)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFolderQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<StorageFolderQueryResult>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateItemQuery(&self) -> ::windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQuery)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateItemQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<StorageItemQueryResult>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).from_abi(result__)
        }
    }
    pub fn AreQueryOptionsSupported<P0>(&self, queryoptions: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreQueryOptionsSupported)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFolderQuerySupported)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
    pub fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFileQuerySupported)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IStorageFolderQueryOperations, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IStorageFolderQueryOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolderQueryOperations {}
impl ::core::fmt::Debug for IStorageFolderQueryOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolderQueryOperations").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IStorageFolderQueryOperations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cb43ccc9-446b-4a4f-be97-757771be5203}");
}
unsafe impl ::windows_core::Interface for IStorageFolderQueryOperations {
    type Vtable = IStorageFolderQueryOperations_Vtbl;
}
impl ::core::clone::Clone for IStorageFolderQueryOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageFolderQueryOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb43ccc9_446b_4a4f_be97_757771be5203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryOperations_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetIndexedStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIndexedStateAsync: usize,
    pub CreateFileQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFileQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFolderQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFolderQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateItemQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateItemQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    pub AreQueryOptionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCommonFolderQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCommonFileQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolderQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
}
impl ::core::clone::Clone for IStorageFolderQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageFolderQueryResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6654c911_7d66_46fa_aecf_e4a4baa93ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
}
impl ::core::clone::Clone for IStorageItemQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageItemQueryResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8948079_9d58_47b8_b2b2_41b07f4795f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IStorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageLibraryChangeTrackerTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dc7a369_b7a3_4df2_9d61_eba85a0343d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IStorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageLibraryContentChangedTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a371977_abbf_4e1d_8aa5_6385d8884799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateModifiedSinceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastquerytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateModifiedSinceQuery: usize,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IStorageQueryResultBase(::windows_core::IUnknown);
impl IStorageQueryResultBase {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IStorageQueryResultBase, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IStorageQueryResultBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageQueryResultBase {}
impl ::core::fmt::Debug for IStorageQueryResultBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageQueryResultBase").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IStorageQueryResultBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{c297d70d-7353-47ab-ba58-8c61425dc54b}");
}
unsafe impl ::windows_core::Interface for IStorageQueryResultBase {
    type Vtable = IStorageQueryResultBase_Vtbl;
}
impl ::core::clone::Clone for IStorageQueryResultBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageQueryResultBase {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc297d70d_7353_47ab_ba58_8c61425dc54b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageQueryResultBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub OptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub FindStartIndexAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindStartIndexAsync: usize,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplyNewQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newqueryoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IValueAndLanguage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
}
impl ::core::clone::Clone for IValueAndLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IValueAndLanguage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9914881_a1ee_4bc4_92a5_466968e30436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueAndLanguage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ContentIndexer(::windows_core::IUnknown);
impl ContentIndexer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddAsync<P0>(&self, indexablecontent: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<IIndexableContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAsync)(::windows_core::Interface::as_raw(this), indexablecontent.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync<P0>(&self, indexablecontent: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<IIndexableContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync)(::windows_core::Interface::as_raw(this), indexablecontent.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, contentid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contentid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteMultipleAsync<P0>(&self, contentids: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteMultipleAsync)(::windows_core::Interface::as_raw(this), contentids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAllAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0>(&self, contentid: &::windows_core::HSTRING, propertiestoretrieve: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePropertiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contentid), propertiestoretrieve.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrderAndLanguage<P0, P1>(&self, searchfilter: &::windows_core::HSTRING, propertiestoretrieve: P0, sortorder: P1, searchfilterlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<ContentIndexerQuery>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SortEntry>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateQueryWithSortOrderAndLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchfilter), propertiestoretrieve.try_into_param()?.abi(), sortorder.try_into_param()?.abi(), ::core::mem::transmute_copy(searchfilterlanguage), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrder<P0, P1>(&self, searchfilter: &::windows_core::HSTRING, propertiestoretrieve: P0, sortorder: P1) -> ::windows_core::Result<ContentIndexerQuery>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<SortEntry>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateQueryWithSortOrder)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchfilter), propertiestoretrieve.try_into_param()?.abi(), sortorder.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQuery<P0>(&self, searchfilter: &::windows_core::HSTRING, propertiestoretrieve: P0) -> ::windows_core::Result<ContentIndexerQuery>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateQuery)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchfilter), propertiestoretrieve.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetIndexerWithName(indexname: &::windows_core::HSTRING) -> ::windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexerWithName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(indexname), &mut result__).from_abi(result__)
        })
    }
    pub fn GetIndexer() -> ::windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentIndexerStatics<R, F: FnOnce(&IContentIndexerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContentIndexer, IContentIndexerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ContentIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIndexer {}
impl ::core::fmt::Debug for ContentIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIndexer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentIndexer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexer;{b1767f8d-f698-4982-b05f-3a6e8cab01a2})");
}
impl ::core::clone::Clone for ContentIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIndexer {
    const IID: ::windows_core::GUID = <IContentIndexer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexer";
}
::windows_core::imp::interface_hierarchy!(ContentIndexer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContentIndexer {}
unsafe impl ::core::marker::Sync for ContentIndexer {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ContentIndexerQuery(::windows_core::IUnknown);
impl ContentIndexerQuery {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesRangeAsync)(::windows_core::Interface::as_raw(this), startindex, maxitems, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRangeAsync)(::windows_core::Interface::as_raw(this), startindex, maxitems, &mut result__).from_abi(result__)
        }
    }
    pub fn QueryFolder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryFolder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContentIndexerQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIndexerQuery {}
impl ::core::fmt::Debug for ContentIndexerQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIndexerQuery").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentIndexerQuery {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexerQuery;{70e3b0f8-4bfc-428a-8889-cc51da9a7b9d})");
}
impl ::core::clone::Clone for ContentIndexerQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIndexerQuery {
    const IID: ::windows_core::GUID = <IContentIndexerQuery as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexerQuery";
}
::windows_core::imp::interface_hierarchy!(ContentIndexerQuery, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ContentIndexerQuery {}
unsafe impl ::core::marker::Sync for ContentIndexerQuery {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IndexableContent(::windows_core::IUnknown);
impl IndexableContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IndexableContent, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StreamContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStreamContentType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamContentType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for IndexableContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IndexableContent {}
impl ::core::fmt::Debug for IndexableContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexableContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IndexableContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.IndexableContent;{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4})");
}
impl ::core::clone::Clone for IndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IndexableContent {
    type Vtable = IIndexableContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IndexableContent {
    const IID: ::windows_core::GUID = <IIndexableContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IndexableContent";
}
::windows_core::imp::interface_hierarchy!(IndexableContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IIndexableContent> for IndexableContent {}
unsafe impl ::core::marker::Send for IndexableContent {}
unsafe impl ::core::marker::Sync for IndexableContent {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct QueryOptions(::windows_core::IUnknown);
impl QueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<QueryOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileTypeFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FolderDepth(&self) -> ::windows_core::Result<FolderDepth> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FolderDepth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFolderDepth(&self, value: FolderDepth) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFolderDepth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationSearchFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationSearchFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetApplicationSearchFilter(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetApplicationSearchFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserSearchFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserSearchFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserSearchFilter(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserSearchFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IndexerOption(&self) -> ::windows_core::Result<IndexerOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexerOption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIndexerOption(&self, value: IndexerOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIndexerOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SortOrder(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SortOrder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GroupPropertyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GroupPropertyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DateStackOption(&self) -> ::windows_core::Result<DateStackOption> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DateStackOption)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SaveToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveToString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LoadFromString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailPrefetch)(::windows_core::Interface::as_raw(this), mode, requestedsize, options).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub fn SetPropertyPrefetch<P0>(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPropertyPrefetch)(::windows_core::Interface::as_raw(this), options, propertiestoretrieve.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCommonFileQuery<P0>(query: CommonFileQuery, filetypefilter: P0) -> ::windows_core::Result<QueryOptions>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCommonFileQuery)(::windows_core::Interface::as_raw(this), query, filetypefilter.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateCommonFolderQuery(query: CommonFolderQuery) -> ::windows_core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCommonFolderQuery)(::windows_core::Interface::as_raw(this), query, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderIdFilter(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IQueryOptionsWithProviderFilter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StorageProviderIdFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IQueryOptionsFactory<R, F: FnOnce(&IQueryOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<QueryOptions, IQueryOptionsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for QueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QueryOptions {}
impl ::core::fmt::Debug for QueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for QueryOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.QueryOptions;{1e5e46ee-0f45-4838-a8e9-d0479d446c30})");
}
impl ::core::clone::Clone for QueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for QueryOptions {
    type Vtable = IQueryOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for QueryOptions {
    const IID: ::windows_core::GUID = <IQueryOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for QueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.QueryOptions";
}
::windows_core::imp::interface_hierarchy!(QueryOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for QueryOptions {}
unsafe impl ::core::marker::Sync for QueryOptions {}
#[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SortEntryVector(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SortEntryVector {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<SortEntry>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<SortEntry>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SortEntry> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<SortEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SortEntry>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SortEntry>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SortEntry>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [SortEntry]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[SortEntry]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SortEntryVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SortEntryVector {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for SortEntryVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SortEntryVector").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for SortEntryVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.SortEntryVector;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Storage.Search.SortEntry;string;b1)))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SortEntryVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for SortEntryVector {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<SortEntry>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for SortEntryVector {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVector<SortEntry> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for SortEntryVector {
    const NAME: &'static str = "Windows.Storage.Search.SortEntryVector";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for SortEntryVector {
    type Item = SortEntry;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &SortEntryVector {
    type Item = SortEntry;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(SortEntryVector, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<SortEntry>> for SortEntryVector {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVector<SortEntry>> for SortEntryVector {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageFileQueryResult(::windows_core::IUnknown);
impl StorageFileQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub fn GetMatchingPropertiesWithRanges<P0>(&self, file: P0) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>>
    where
        P0: ::windows_core::IntoParam<super::StorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageFileQueryResult2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMatchingPropertiesWithRanges)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageFileQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFileQueryResult {}
impl ::core::fmt::Debug for StorageFileQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFileQueryResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageFileQueryResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFileQueryResult;{52fda447-2baa-412c-b29f-d4b1778efa1e})");
}
impl ::core::clone::Clone for StorageFileQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageFileQueryResult {
    const IID: ::windows_core::GUID = <IStorageFileQueryResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFileQueryResult";
}
::windows_core::imp::interface_hierarchy!(StorageFileQueryResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IStorageQueryResultBase> for StorageFileQueryResult {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageFolderQueryResult(::windows_core::IUnknown);
impl StorageFolderQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageFolderQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFolderQueryResult {}
impl ::core::fmt::Debug for StorageFolderQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFolderQueryResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageFolderQueryResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFolderQueryResult;{6654c911-7d66-46fa-aecf-e4a4baa93ab8})");
}
impl ::core::clone::Clone for StorageFolderQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageFolderQueryResult {
    const IID: ::windows_core::GUID = <IStorageFolderQueryResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFolderQueryResult";
}
::windows_core::imp::interface_hierarchy!(StorageFolderQueryResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IStorageQueryResultBase> for StorageFolderQueryResult {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageItemQueryResult(::windows_core::IUnknown);
impl StorageItemQueryResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageItemQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemQueryResult {}
impl ::core::fmt::Debug for StorageItemQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemQueryResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageItemQueryResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageItemQueryResult;{e8948079-9d58-47b8-b2b2-41b07f4795f9})");
}
impl ::core::clone::Clone for StorageItemQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageItemQueryResult {
    const IID: ::windows_core::GUID = <IStorageItemQueryResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageItemQueryResult";
}
::windows_core::imp::interface_hierarchy!(StorageItemQueryResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IStorageQueryResultBase> for StorageItemQueryResult {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTriggerDetails(::windows_core::IUnknown);
impl StorageLibraryChangeTrackerTriggerDetails {
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows_core::Result<super::StorageLibraryChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTrackerTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTrackerTriggerDetails {}
impl ::core::fmt::Debug for StorageLibraryChangeTrackerTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeTrackerTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageLibraryChangeTrackerTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails;{1dc7a369-b7a3-4df2-9d61-eba85a0343d2})");
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageLibraryChangeTrackerTriggerDetails {
    const IID: ::windows_core::GUID = <IStorageLibraryChangeTrackerTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(StorageLibraryChangeTrackerTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageLibraryContentChangedTriggerDetails(::windows_core::IUnknown);
impl StorageLibraryContentChangedTriggerDetails {
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateModifiedSinceQuery(&self, lastquerytime: super::super::Foundation::DateTime) -> ::windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateModifiedSinceQuery)(::windows_core::Interface::as_raw(this), lastquerytime, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StorageLibraryContentChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryContentChangedTriggerDetails {}
impl ::core::fmt::Debug for StorageLibraryContentChangedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryContentChangedTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageLibraryContentChangedTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails;{2a371977-abbf-4e1d-8aa5-6385d8884799})");
}
impl ::core::clone::Clone for StorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageLibraryContentChangedTriggerDetails {
    const IID: ::windows_core::GUID = <IStorageLibraryContentChangedTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(StorageLibraryContentChangedTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ValueAndLanguage(::windows_core::IUnknown);
impl ValueAndLanguage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ValueAndLanguage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for ValueAndLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValueAndLanguage {}
impl ::core::fmt::Debug for ValueAndLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueAndLanguage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ValueAndLanguage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ValueAndLanguage;{b9914881-a1ee-4bc4-92a5-466968e30436})");
}
impl ::core::clone::Clone for ValueAndLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ValueAndLanguage {
    const IID: ::windows_core::GUID = <IValueAndLanguage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.ValueAndLanguage";
}
::windows_core::imp::interface_hierarchy!(ValueAndLanguage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ValueAndLanguage {}
unsafe impl ::core::marker::Sync for ValueAndLanguage {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const OrderByName: Self = Self(1i32);
    pub const OrderByTitle: Self = Self(2i32);
    pub const OrderByMusicProperties: Self = Self(3i32);
    pub const OrderBySearchRank: Self = Self(4i32);
    pub const OrderByDate: Self = Self(5i32);
}
impl ::core::marker::Copy for CommonFileQuery {}
impl ::core::clone::Clone for CommonFileQuery {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommonFileQuery {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CommonFileQuery {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CommonFileQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFileQuery").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CommonFileQuery {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFileQuery;i4)");
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const GroupByYear: Self = Self(100i32);
    pub const GroupByMonth: Self = Self(101i32);
    pub const GroupByArtist: Self = Self(102i32);
    pub const GroupByAlbum: Self = Self(103i32);
    pub const GroupByAlbumArtist: Self = Self(104i32);
    pub const GroupByComposer: Self = Self(105i32);
    pub const GroupByGenre: Self = Self(106i32);
    pub const GroupByPublishedYear: Self = Self(107i32);
    pub const GroupByRating: Self = Self(108i32);
    pub const GroupByTag: Self = Self(109i32);
    pub const GroupByAuthor: Self = Self(110i32);
    pub const GroupByType: Self = Self(111i32);
}
impl ::core::marker::Copy for CommonFolderQuery {}
impl ::core::clone::Clone for CommonFolderQuery {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommonFolderQuery {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CommonFolderQuery {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CommonFolderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFolderQuery").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CommonFolderQuery {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFolderQuery;i4)");
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Month: Self = Self(2i32);
}
impl ::core::marker::Copy for DateStackOption {}
impl ::core::clone::Clone for DateStackOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DateStackOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DateStackOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DateStackOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateStackOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DateStackOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.DateStackOption;i4)");
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: Self = Self(0i32);
    pub const Deep: Self = Self(1i32);
}
impl ::core::marker::Copy for FolderDepth {}
impl ::core::clone::Clone for FolderDepth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FolderDepth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FolderDepth {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FolderDepth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderDepth").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FolderDepth {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.FolderDepth;i4)");
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0i32);
    pub const NotIndexed: Self = Self(1i32);
    pub const PartiallyIndexed: Self = Self(2i32);
    pub const FullyIndexed: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexedState {}
impl ::core::clone::Clone for IndexedState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IndexedState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IndexedState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IndexedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IndexedState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexedState;i4)");
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: Self = Self(0i32);
    pub const OnlyUseIndexer: Self = Self(1i32);
    pub const DoNotUseIndexer: Self = Self(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexerOption {}
impl ::core::clone::Clone for IndexerOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IndexerOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IndexerOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IndexerOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexerOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IndexerOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexerOption;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Storage_Search\"`*"]
pub struct SortEntry {
    pub PropertyName: ::windows_core::HSTRING,
    pub AscendingOrder: bool,
}
impl ::core::clone::Clone for SortEntry {
    fn clone(&self) -> Self {
        Self { PropertyName: self.PropertyName.clone(), AscendingOrder: self.AscendingOrder }
    }
}
impl ::core::fmt::Debug for SortEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SortEntry").field("PropertyName", &self.PropertyName).field("AscendingOrder", &self.AscendingOrder).finish()
    }
}
impl ::windows_core::TypeKind for SortEntry {
    type TypeKind = ::windows_core::ValueType;
}
impl ::windows_core::RuntimeType for SortEntry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Storage.Search.SortEntry;string;b1)");
}
impl ::core::cmp::PartialEq for SortEntry {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyName == other.PropertyName && self.AscendingOrder == other.AscendingOrder
    }
}
impl ::core::cmp::Eq for SortEntry {}
impl ::core::default::Default for SortEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
